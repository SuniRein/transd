mod source;
mod target;
mod toolbar;

use super::message::Message;
use super::state::AppState;
use iced::widget::{column, container, responsive, row, rule};
use iced::{Element, Length::Fill};

pub fn view(state: &AppState) -> Element<'_, Message> {
    responsive(|size| {
        let is_narrow = size.width < 760.0;

        let toolbar = toolbar::view_toolbar(state);
        let source = source::view_source(state);
        let target = target::view_target(state);

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
            let panes = row![source, target].spacing(12).height(Fill);

            column![toolbar, rule::horizontal(1), panes]
                .spacing(12)
                .padding(12)
                .into()
        };

        container(content).width(Fill).height(Fill).into()
    })
    .into()
}
