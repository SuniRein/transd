#![warn(clippy::pedantic, clippy::nursery)]

use async_trait::async_trait;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Language {
    pub id: String,
    pub name: String,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Engine {
    pub id: String,
    pub name: String,
}

impl fmt::Display for Engine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[async_trait]
pub trait Translator: Send + Sync {
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
