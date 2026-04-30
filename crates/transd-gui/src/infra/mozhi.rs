use rootcause::prelude::*;
use std::{env, sync::Arc};
use transd_provider_mozhi::MozhiTranslator;
use transd_translate::Translator;

/// Creates the default translator backend.
///
/// NOTE: for now we keep the existing behaviour:
/// `MOZHI_INSTANCE` must be set or the app will exit.
pub fn build_translator_from_env() -> Arc<dyn Translator<Error = Report>> {
    let instance =
        env::var("MOZHI_INSTANCE").expect("MOZHI_INSTANCE environment variable is not set");
    Arc::new(MozhiTranslator::new(instance))
}
