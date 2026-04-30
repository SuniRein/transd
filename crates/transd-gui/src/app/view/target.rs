use super::{AppState, Message};
use iced::widget::{column, container, scrollable, text};
use iced::{Element, Length::Fill};

pub fn view_target(state: &AppState) -> Element<'_, Message> {
    let content = state.target.text();

    let body: Element<'_, Message> = if content.trim().is_empty() {
        scrollable(text("Translation result will appear here.").style(text::secondary))
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
