#![warn(clippy::pedantic, clippy::nursery)]

mod app;
mod infra;

use crate::app::App;
use std::sync::Arc;

fn main() -> iced::Result {
    let translator = infra::mozhi::build_translator_from_env();

    iced::application(
        move || App::new(Arc::clone(&translator)),
        App::update,
        App::view,
    )
    .title("transd")
    .centered()
    .run()
}
