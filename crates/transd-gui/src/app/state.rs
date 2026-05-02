use iced::widget::text_editor;
use rootcause::markers::{Cloneable, Dynamic};
use rootcause::prelude::*;
use transd_translate::{Engine, Language};

#[derive(Debug)]
pub struct AppState {
    pub engine: Option<Engine>,
    pub source_language: Option<Language>,
    pub target_language: Option<Language>,

    pub available_engines: Vec<Engine>,
    pub available_source_languages: Vec<Language>,
    pub available_target_languages: Vec<Language>,

    pub source: text_editor::Content,
    pub target: text_editor::Content,

    pub is_loading: bool,
    pub pending: usize,
    pub error: Option<Report<Dynamic, Cloneable>>,

    /// Human readable status line (e.g. `DBus` startup/errors).
    pub status: Option<String>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
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
            status: None,
        }
    }

    pub fn can_swap(&self) -> bool {
        if let (Some(source), Some(target)) = (&self.source_language, &self.target_language) {
            return find_language(&self.available_source_languages, target).is_some()
                && find_language(&self.available_target_languages, source).is_some();
        }
        false
    }

    pub fn can_translate(&self) -> bool {
        !self.is_loading
            && self.engine.is_some()
            && self.source_language.is_some()
            && self.target_language.is_some()
            && !self.source.text().trim().is_empty()
    }
}

pub fn find_engine(available: &[Engine], selected: &Engine) -> Option<Engine> {
    available.iter().find(|e| e.id == selected.id).cloned()
}

pub fn find_language(available: &[Language], selected: &Language) -> Option<Language> {
    available.iter().find(|l| l.id == selected.id).cloned()
}
