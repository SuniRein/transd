use async_trait::async_trait;

pub struct Language {
    pub id: String,
    pub name: String,
}

pub struct Engine {
    pub id: String,
    pub name: String,
}

#[async_trait]
pub trait Translator {
    type Error;

    async fn translate(
        &self,
        text: &str,
        engine: &str,
        from: &str,
        to: &str,
    ) -> Result<String, Self::Error>;

    async fn list_engines(&self) -> Result<Vec<Engine>, Self::Error>;
    async fn list_source_languages(&self, engine: &str) -> Result<Vec<Language>, Self::Error>;
    async fn list_target_languages(&self, engine: &str) -> Result<Vec<Language>, Self::Error>;
}
