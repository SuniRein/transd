use super::{AppState, Message};
use iced::widget::{Space, button, column, container, pick_list, row, text};
use iced::{Element, Length::Fill, alignment::Alignment};

pub fn view_toolbar(state: &AppState) -> Element<'_, Message> {
    let selectors = {
        let engine = state.available_engines.clone();
        let selected_engine = state.engine.clone();

        let source = state.available_source_languages.clone();
        let selected_source = state.source_language.clone();

        let target = state.available_target_languages.clone();
        let selected_target = state.target_language.clone();

        let swap_button = if state.can_swap() {
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
        let translate = if state.can_translate() {
            button("Translate")
                .style(button::primary)
                .on_press(Message::Translate)
        } else {
            button("Translate")
        };

        let copy = if state.target.text().trim().is_empty() {
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
        let status = if state.is_loading {
            Some("Loading...".to_string())
        } else {
            state.error.as_ref().map(ToString::to_string)
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
