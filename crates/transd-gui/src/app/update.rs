use super::{
    Message, service,
    state::{self, AppState},
};
use iced::{Task, widget::text_editor};
use rootcause::prelude::*;
use std::sync::Arc;
use transd_translate::Translator;

pub fn update(
    state: &mut AppState,
    message: Message,
    translator: Arc<dyn Translator<Error = Report>>,
) -> Task<Message> {
    match message {
        Message::EnginesLoaded(result) => {
            state.pending = state.pending.saturating_sub(1);
            state.is_loading = state.pending > 0;
            match result {
                Ok(engines) => {
                    state.available_engines = engines;
                    // Prefer the first engine as default.
                    state.engine = state.available_engines.first().cloned();
                    state.available_source_languages.clear();
                    state.available_target_languages.clear();
                    state.source_language = None;
                    state.target_language = None;
                    service::load_languages(state, translator)
                }
                Err(e) => {
                    state.error = Some(e);
                    Task::none()
                }
            }
        }

        Message::EngineSelected(selected) => {
            let Some(engine) = state::find_engine(&state.available_engines, &selected) else {
                return Task::none();
            };

            if state.engine.as_ref().is_some_and(|e| e.id == engine.id) {
                return Task::none();
            }

            state.engine = Some(engine);
            state.available_source_languages.clear();
            state.available_target_languages.clear();
            state.source_language = None;
            state.target_language = None;
            service::load_languages(state, translator)
        }

        Message::SourceLanguagesLoaded(result) => {
            state.pending = state.pending.saturating_sub(1);
            state.is_loading = state.pending > 0;
            match result {
                Ok(langs) => {
                    state.available_source_languages = langs;
                    state.source_language = state.available_source_languages.first().cloned();
                    state.error = None;
                }
                Err(e) => {
                    state.error = Some(e);
                }
            }
            Task::none()
        }

        Message::TargetLanguagesLoaded(result) => {
            state.pending = state.pending.saturating_sub(1);
            state.is_loading = state.pending > 0;
            match result {
                Ok(langs) => {
                    state.available_target_languages = langs;
                    state.target_language = state.available_target_languages.first().cloned();
                    state.error = None;
                }
                Err(e) => {
                    state.error = Some(e);
                }
            }
            Task::none()
        }

        Message::SourceLanguageSelected(selected) => {
            let Some(lang) = state::find_language(&state.available_source_languages, &selected)
            else {
                return Task::none();
            };
            state.source_language = Some(lang);
            Task::none()
        }

        Message::TargetLanguageSelected(selected) => {
            let Some(lang) = state::find_language(&state.available_target_languages, &selected)
            else {
                return Task::none();
            };
            state.target_language = Some(lang);
            Task::none()
        }

        Message::SourceEdited(action) => {
            state.source.perform(action);
            Task::none()
        }

        Message::SwapLanguages => {
            if !state.can_swap() {
                return Task::none();
            }
            std::mem::swap(&mut state.source_language, &mut state.target_language);
            Task::none()
        }

        Message::Clear => {
            state.source = text_editor::Content::new();
            state.target = text_editor::Content::new();
            state.error = None;
            Task::none()
        }

        Message::Translate => service::translate(state, translator),

        Message::Translated(result) => {
            state.pending = state.pending.saturating_sub(1);
            state.is_loading = state.pending > 0;
            match result {
                Ok(output) => {
                    state.target = text_editor::Content::with_text(&output);
                    state.error = None;
                }
                Err(e) => {
                    state.error = Some(e);
                }
            }
            Task::none()
        }

        Message::CopyResult => {
            let text = state.target.text();
            if text.trim().is_empty() {
                return Task::none();
            }
            iced::clipboard::write(text)
        }
    }
}
