use iced::widget::{
    Space, button, column, container, pick_list, responsive, row, rule, scrollable, text,
    text_editor,
};
use iced::{Element, Length::Fill, Task, alignment::Alignment};
use rootcause::markers::{Cloneable, Dynamic};
use rootcause::prelude::*;
use std::{
    env,
    sync::{Arc, LazyLock},
};
use transd_provider_mozhi::MozhiTranslator;
use transd_translate::{Engine, Language, Translator};

static MOZHI_INSTANCE: LazyLock<String> = LazyLock::new(|| {
    env::var("MOZHI_INSTANCE").expect("MOZHI_INSTANCE environment variable is not set")
});

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title("transd")
        .centered()
        .run()
}

struct App {
    translator: Arc<dyn Translator<Error = Report>>,

    engine: Option<Engine>,
    source_language: Option<Language>,
    target_language: Option<Language>,

    available_engines: Vec<Engine>,
    available_source_languages: Vec<Language>,
    available_target_languages: Vec<Language>,

    source: text_editor::Content,
    target: text_editor::Content,

    is_loading: bool,
    pending: usize,
    error: Option<Report<Dynamic, Cloneable>>,
}

#[derive(Debug, Clone)]
enum Message {
    EnginesLoaded(Result<Vec<Engine>, Report<Dynamic, Cloneable>>),
    EngineSelected(Engine),
    SourceLanguagesLoaded(Result<Vec<Language>, Report<Dynamic, Cloneable>>),
    TargetLanguagesLoaded(Result<Vec<Language>, Report<Dynamic, Cloneable>>),
    SourceLanguageSelected(Language),
    TargetLanguageSelected(Language),

    SourceEdited(text_editor::Action),

    SwapLanguages,
    Clear,
    Translate,
    Translated(Result<String, Report<Dynamic, Cloneable>>),
    CopyResult,
}

impl App {
    fn new() -> (Self, Task<Message>) {
        let translator = MozhiTranslator::new(MOZHI_INSTANCE.clone());

        let mut app = Self {
            translator: Arc::new(translator),

            engine: None,
            source_language: None,
            target_language: None,
            available_engines: Vec::new(),
            available_source_languages: Vec::new(),
            available_target_languages: Vec::new(),

            source: text_editor::Content::new(),
            target: text_editor::Content::new(),

            is_loading: true,
            pending: 0,
            error: None,
        };

        let task = app.load_engines();
        (app, task)
    }

    fn view(&self) -> Element<'_, Message> {
        responsive(|size| {
            let is_narrow = size.width < 760.0;

            let toolbar = self.view_toolbar();
            let source = self.view_source();
            let target = self.view_target();

            let content: Element<'_, Message> = if is_narrow {
                column![
                    toolbar,
                    rule::horizontal(1),
                    source,
                    rule::horizontal(1),
                    target
                ]
                .spacing(12)
                .padding(12)
                .into()
            } else {
                let panes: Element<'_, Message> =
                    row![source, target].spacing(12).height(Fill).into();

                column![toolbar, rule::horizontal(1), panes]
                    .spacing(12)
                    .padding(12)
                    .into()
            };

            container(content).width(Fill).height(Fill).into()
        })
        .into()
    }

    fn view_toolbar(&self) -> Element<'_, Message> {
        let selectors = {
            let engine = self.available_engines.clone();
            let selected_engine = self.engine.clone();

            let source = self.available_source_languages.clone();
            let selected_source = self.source_language.clone();

            let target = self.available_target_languages.clone();
            let selected_target = self.target_language.clone();

            let swap_button = if self.can_swap(&self.source_language, &self.target_language) {
                button("Swap").on_press(Message::SwapLanguages)
            } else {
                button("Swap")
            };

            row![
                pick_list(engine, selected_engine, Message::EngineSelected)
                    .placeholder("Engine")
                    .width(160),
                pick_list(source, selected_source, Message::SourceLanguageSelected)
                    .placeholder("From")
                    .width(160),
                swap_button,
                pick_list(target, selected_target, Message::TargetLanguageSelected)
                    .placeholder("To")
                    .width(160),
            ]
            .spacing(8)
            .align_y(Alignment::Center)
        };

        let actions = {
            let can_translate = !self.is_loading
                && self.engine.is_some()
                && self.source_language.is_some()
                && self.target_language.is_some()
                && !self.source.text().trim().is_empty();

            let translate = if can_translate {
                button("Translate")
                    .style(button::primary)
                    .on_press(Message::Translate)
            } else {
                button("Translate")
            };

            let copy = if self.target.text().trim().is_empty() {
                button("Copy")
            } else {
                button("Copy").on_press(Message::CopyResult)
            };

            row![translate, button("Clear").on_press(Message::Clear), copy]
                .spacing(8)
                .align_y(Alignment::Center)
        };

        let top = row![selectors, Space::new().width(Fill), actions]
            .spacing(16)
            .align_y(Alignment::Center);

        let status = {
            let status = if self.is_loading {
                Some("Loading...".to_string())
            } else {
                self.error.as_ref().map(|e| e.to_string())
            };

            status.map(|s| {
                container(text(s).size(12))
                    .padding([2, 8])
                    .style(container::bordered_box)
            })
        };

        let mut header = column![top].spacing(8);
        if let Some(status) = status {
            header = header.push(status);
        }

        header.into()
    }

    fn can_swap(&self, source: &Option<Language>, target: &Option<Language>) -> bool {
        if let (Some(source), Some(target)) = (source, target) {
            return find_language(&self.available_source_languages, target).is_some()
                && find_language(&self.available_target_languages, source).is_some();
        }
        false
    }

    fn view_source(&self) -> Element<'_, Message> {
        let editor = text_editor(&self.source)
            .placeholder("Type text to translate...")
            .on_action(Message::SourceEdited)
            .height(Fill);

        container(column![text("Source").size(14), editor].spacing(8))
            .width(Fill)
            .height(Fill)
            .padding(12)
            .style(container::bordered_box)
            .into()
    }

    fn view_target(&self) -> Element<'_, Message> {
        let content = self.target.text();

        let body: Element<'_, Message> = if content.trim().is_empty() {
            scrollable(
                text("Translation result will appear here.").style(iced::widget::text::secondary),
            )
            .height(Fill)
            .width(Fill)
            .into()
        } else {
            scrollable(text(content).width(Fill))
                .height(Fill)
                .width(Fill)
                .into()
        };

        container(column![text("Target").size(14), body].spacing(8))
            .width(Fill)
            .height(Fill)
            .padding(12)
            .style(container::bordered_box)
            .into()
    }

    fn load_engines(&mut self) -> Task<Message> {
        self.pending += 1;
        self.is_loading = true;
        self.error = None;

        let translator = Arc::clone(&self.translator);

        Task::perform(async move { translator.list_engines().await }, |res| {
            Message::EnginesLoaded(res.map_err(|e| e.into_cloneable()))
        })
    }

    fn load_languages(&mut self) -> Task<Message> {
        let Some(engine) = self.engine.as_ref() else {
            return Task::none();
        };

        self.pending += 2;
        self.is_loading = true;
        self.error = None;

        let engine_id = engine.id.clone();

        let translator1 = Arc::clone(&self.translator);
        let translator2 = Arc::clone(&self.translator);

        let source_task = Task::perform(
            async move { translator1.list_source_languages(&engine_id).await },
            |res| Message::SourceLanguagesLoaded(res.map_err(|e| e.into_cloneable())),
        );

        let engine_id = engine.id.clone();
        let target_task = Task::perform(
            async move { translator2.list_target_languages(&engine_id).await },
            |res| Message::TargetLanguagesLoaded(res.map_err(|e| e.into_cloneable())),
        );

        Task::batch([source_task, target_task])
    }

    fn translate(&mut self) -> Task<Message> {
        let Some(engine) = self.engine.as_ref() else {
            return Task::none();
        };
        let Some(from) = self.source_language.as_ref() else {
            return Task::none();
        };
        let Some(to) = self.target_language.as_ref() else {
            return Task::none();
        };

        let text = self.source.text();
        if text.trim().is_empty() {
            return Task::none();
        }

        self.pending += 1;
        self.is_loading = true;
        self.error = None;

        let engine = engine.id.clone();
        let from = from.id.clone();
        let to = to.id.clone();
        let translator = Arc::clone(&self.translator);

        Task::perform(
            async move { translator.translate(&text, &engine, &from, &to).await },
            |res| Message::Translated(res.map_err(|e| e.into_cloneable())),
        )
    }
}

fn find_engine(available: &[Engine], selected: &Engine) -> Option<Engine> {
    available.iter().find(|e| e.id == selected.id).cloned()
}

fn find_language(available: &[Language], selected: &Language) -> Option<Language> {
    available.iter().find(|l| l.id == selected.id).cloned()
}

impl App {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::EnginesLoaded(result) => {
                self.pending = self.pending.saturating_sub(1);
                self.is_loading = self.pending > 0;
                match result {
                    Ok(engines) => {
                        self.available_engines = engines;
                        // Prefer the first engine as default.
                        self.engine = self.available_engines.first().cloned();
                        self.available_source_languages.clear();
                        self.available_target_languages.clear();
                        self.source_language = None;
                        self.target_language = None;
                        self.load_languages()
                    }
                    Err(e) => {
                        self.error = Some(e);
                        Task::none()
                    }
                }
            }

            Message::EngineSelected(selected) => {
                let Some(engine) = find_engine(&self.available_engines, &selected) else {
                    return Task::none();
                };

                if self.engine.as_ref().is_some_and(|e| e.id == engine.id) {
                    return Task::none();
                }

                self.engine = Some(engine);
                self.available_source_languages.clear();
                self.available_target_languages.clear();
                self.source_language = None;
                self.target_language = None;
                self.load_languages()
            }

            Message::SourceLanguagesLoaded(result) => {
                self.pending = self.pending.saturating_sub(1);
                self.is_loading = self.pending > 0;
                match result {
                    Ok(langs) => {
                        self.available_source_languages = langs;
                        self.source_language = self.available_source_languages.first().cloned();
                        self.error = None;
                    }
                    Err(e) => {
                        self.error = Some(e);
                    }
                }
                Task::none()
            }

            Message::TargetLanguagesLoaded(result) => {
                self.pending = self.pending.saturating_sub(1);
                self.is_loading = self.pending > 0;
                match result {
                    Ok(langs) => {
                        self.available_target_languages = langs;
                        self.target_language = self.available_target_languages.first().cloned();
                        self.error = None;
                    }
                    Err(e) => {
                        self.error = Some(e);
                    }
                }
                Task::none()
            }

            Message::SourceLanguageSelected(selected) => {
                let Some(lang) = find_language(&self.available_source_languages, &selected) else {
                    return Task::none();
                };
                self.source_language = Some(lang);
                Task::none()
            }

            Message::TargetLanguageSelected(selected) => {
                let Some(lang) = find_language(&self.available_target_languages, &selected) else {
                    return Task::none();
                };
                self.target_language = Some(lang);
                Task::none()
            }

            Message::SourceEdited(action) => {
                self.source.perform(action);
                Task::none()
            }

            Message::SwapLanguages => {
                if !self.can_swap(&self.source_language, &self.target_language) {
                    return Task::none();
                }
                std::mem::swap(&mut self.source_language, &mut self.target_language);
                Task::none()
            }

            Message::Clear => {
                self.source = text_editor::Content::new();
                self.target = text_editor::Content::new();
                self.error = None;
                Task::none()
            }

            Message::Translate => self.translate(),

            Message::Translated(result) => {
                self.pending = self.pending.saturating_sub(1);
                self.is_loading = self.pending > 0;
                match result {
                    Ok(output) => {
                        self.target = text_editor::Content::with_text(&output);
                        self.error = None;
                    }
                    Err(e) => {
                        self.error = Some(e);
                    }
                }
                Task::none()
            }

            Message::CopyResult => {
                let text = self.target.text();
                if text.trim().is_empty() {
                    return Task::none();
                }
                iced::clipboard::write(text)
            }
        }
    }
}
