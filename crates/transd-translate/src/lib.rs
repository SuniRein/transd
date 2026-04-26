use async_trait::async_trait;

#[async_trait]
pub trait Translator {
    type Error;

    async fn translate(&self, text: &str, from: &str, to: &str) -> Result<String, Self::Error>;
}
