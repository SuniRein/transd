use crate::app::message::Message;
use crate::app::state::AppState;
use iced::Task;
use rootcause::prelude::*;
use std::sync::Arc;
use transd_translate::Translator;

pub fn load_engines(
    state: &mut AppState,
    translator: Arc<dyn Translator<Error = Report>>,
) -> Task<Message> {
    state.pending += 1;
    state.is_loading = true;
    state.error = None;

    Task::perform(async move { translator.list_engines().await }, |res| {
        Message::EnginesLoaded(res.map_err(Report::into_cloneable))
    })
}

pub fn load_languages(
    state: &mut AppState,
    translator: Arc<dyn Translator<Error = Report>>,
) -> Task<Message> {
    let Some(engine) = state.engine.as_ref() else {
        return Task::none();
    };

    state.pending += 2;
    state.is_loading = true;
    state.error = None;

    let source_task = {
        let translator = Arc::clone(&translator);
        let engine_id = engine.id.clone();
        Task::perform(
            async move { translator.list_source_languages(&engine_id).await },
            |res| Message::SourceLanguagesLoaded(res.map_err(Report::into_cloneable)),
        )
    };

    let target_task = {
        let engine_id = engine.id.clone();
        Task::perform(
            async move { translator.list_target_languages(&engine_id).await },
            |res| Message::TargetLanguagesLoaded(res.map_err(Report::into_cloneable)),
        )
    };

    Task::batch([source_task, target_task])
}

pub fn translate(
    state: &mut AppState,
    translator: Arc<dyn Translator<Error = Report>>,
) -> Task<Message> {
    let (Some(engine), Some(from), Some(to)) = (
        state.engine.as_ref(),
        state.source_language.as_ref(),
        state.target_language.as_ref(),
    ) else {
        return Task::none();
    };

    let text = state.source.text();
    if text.trim().is_empty() {
        return Task::none();
    }

    state.pending += 1;
    state.is_loading = true;
    state.error = None;

    let engine = engine.id.clone();
    let from = from.id.clone();
    let to = to.id.clone();
    Task::perform(
        async move { translator.translate(&text, &engine, &from, &to).await },
        |res| Message::Translated(res.map_err(Report::into_cloneable)),
    )
}
