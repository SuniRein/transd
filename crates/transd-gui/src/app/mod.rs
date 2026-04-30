mod message;
mod service;
mod state;
mod update;
mod view;

pub use message::Message;

use iced::{Element, Task};
use rootcause::prelude::*;
use std::sync::Arc;
use transd_translate::Translator;

pub struct App {
    state: state::AppState,
    translator: Arc<dyn Translator<Error = Report>>,
}

impl App {
    pub fn new(translator: Arc<dyn Translator<Error = Report>>) -> (Self, Task<Message>) {
        let mut app = Self {
            state: state::AppState::new(),
            translator,
        };

        let task = service::load_engines(&mut app.state, Arc::clone(&app.translator));
        (app, task)
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        update::update(&mut self.state, message, Arc::clone(&self.translator))
    }

    pub fn view(&self) -> Element<'_, Message> {
        view::view(&self.state)
    }
}
