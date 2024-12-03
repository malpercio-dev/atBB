use async_trait::async_trait;
use thiserror::Error;

/// A client for interfacing with a database.
#[derive(Clone)]
pub struct DatabaseClient {}

impl DatabaseClient {
    pub fn new() -> color_eyre::Result<Self> {
        Ok(Self {})
    }
}

#[async_trait]
impl health::Checkable for DatabaseClient {
    type Error = DatabaseError;

    async fn check(&self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn name(&self) -> std::borrow::Cow<str> {
        std::borrow::Cow::Borrowed("database")
    }
}

#[derive(Debug, Error)]
pub enum DatabaseError {}
