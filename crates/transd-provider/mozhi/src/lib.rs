use async_trait::async_trait;
use indexmap::IndexMap;
use reqwest::Client;
use rootcause::prelude::*;
use serde::Deserialize;
use transd_translate::{Engine, Language, Translator};

pub struct MozhiTranslator {
    uri: String,
}

impl MozhiTranslator {
    pub fn new(uri: String) -> Self {
        Self { uri }
    }
}

#[async_trait]
impl Translator for MozhiTranslator {
    type Error = Report;

    async fn translate(&self, text: &str, from: &str, to: &str) -> Result<String, Self::Error> {
        let client = Client::new();
        let response = client
            .get(format!("{}/api/translate", self.uri))
            .query(&[
                ("engine", "google"),
                ("text", text),
                ("from", from),
                ("to", to),
            ])
            .send()
            .await
            .context("Failed to send translation request")?;

        let response_json = response
            .json::<serde_json::Value>()
            .await
            .context("Failed to parse translation response")?;

        let response_text = response_json
            .get("translated-text")
            .and_then(|v| v.as_str())
            .ok_or_else(|| report!("Missing 'translated-text' in response"))?;

        Ok(response_text.to_string())
    }

    async fn list_engines(&self) -> Result<Vec<Engine>, Self::Error> {
        let client = Client::new();
        let response = client
            .get(format!("{}/api/engines", self.uri))
            .send()
            .await
            .context("Failed to send engines request")?;

        let response_map = response
            .json::<IndexMap<String, String>>()
            .await
            .context("Failed to parse engines response")?;

        let engines = response_map
            .into_iter()
            .map(|(id, name)| Engine { id, name })
            .collect();

        Ok(engines)
    }

    async fn list_source_languages(&self, engine: &str) -> Result<Vec<Language>, Self::Error> {
        list_languages(&self.uri, "source", engine).await
    }

    async fn list_target_languages(&self, engine: &str) -> Result<Vec<Language>, Self::Error> {
        list_languages(&self.uri, "target", engine).await
    }
}

#[derive(Deserialize)]
struct LanguageInfo {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Id")]
    id: String,
}

async fn list_languages(uri: &str, kind: &str, engine: &str) -> Result<Vec<Language>, Report> {
    let client = Client::new();
    let response = client
        .get(format!("{uri}/api/{kind}_languages"))
        .query(&[("engine", engine)])
        .send()
        .await
        .context(format!("Failed to send {kind} languages request"))?;

    let response_info = response
        .json::<Vec<LanguageInfo>>()
        .await
        .context(format!("Failed to parse {kind} languages response"))?;

    let languages = response_info
        .into_iter()
        .map(|info| Language {
            id: info.id,
            name: info.name,
        })
        .collect();

    Ok(languages)
}
