use rootcause::markers::{Cloneable, Dynamic};
use rootcause::prelude::*;
use transd_translate::{Engine, Language};

#[derive(Debug, Clone)]
pub enum Message {
    EnginesLoaded(Result<Vec<Engine>, Report<Dynamic, Cloneable>>),
    EngineSelected(Engine),
    SourceLanguagesLoaded(Result<Vec<Language>, Report<Dynamic, Cloneable>>),
    TargetLanguagesLoaded(Result<Vec<Language>, Report<Dynamic, Cloneable>>),
    SourceLanguageSelected(Language),
    TargetLanguageSelected(Language),

    SourceEdited(iced::widget::text_editor::Action),

    SwapLanguages,
    Clear,
    Translate,
    Translated(Result<String, Report<Dynamic, Cloneable>>),
    CopyResult,
}
