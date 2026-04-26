use async_trait::async_trait;
use reqwest::Client;
use rootcause::prelude::*;
use transd_translate::Translator;

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
}
