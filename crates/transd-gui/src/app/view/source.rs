use super::{AppState, Message};
use iced::widget::{column, container, text, text_editor};
use iced::{Element, Length::Fill};

pub fn view_source(state: &AppState) -> Element<'_, Message> {
    let editor = text_editor(&state.source)
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
