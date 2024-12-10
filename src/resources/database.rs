use std::str::FromStr;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::{any::install_default_drivers, Any, AnyPool, Pool};
use thiserror::Error;

#[derive(PartialEq, Clone, Copy)]
pub enum DatabaseKind {
    Sqlite,
    Postgres,
}

#[derive(Debug, Error)]
#[error("{0}")]
pub struct DatabaseKindParseError(&'static str);

impl std::str::FromStr for DatabaseKind {
    type Err = DatabaseKindParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "sqlite" => Ok(Self::Sqlite),
            "postgres" => Ok(Self::Postgres),
            _ => Err(DatabaseKindParseError("could not parse DatabaseKind")),
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct DatabaseUrl(pub Box<str>);

impl std::str::FromStr for DatabaseUrl {
    type Err = DatabaseError;

    fn from_str(inner: &str) -> Result<Self, Self::Err> {
        Ok(Self(Box::from(inner)))
    }
}

/// A client for interfacing with a database.
pub struct DatabaseClient {
    pool: Pool<Any>,
    kind: DatabaseKind,
}

impl DatabaseClient {
    pub async fn new(
        database_kind: DatabaseKind,
        database_url: DatabaseUrl,
    ) -> color_eyre::Result<Self> {
        install_default_drivers();
        let opts = sqlx::any::AnyConnectOptions::from_str(&database_url.0)?;
        let pool = AnyPool::connect_with(opts).await.unwrap();
        Ok(Self {
            pool,
            kind: database_kind,
        })
    }

    pub async fn migrate(&self) -> color_eyre::Result<(), sqlx::Error> {
        let mut conn = self.pool.acquire().await?;
        match self.kind {
            DatabaseKind::Postgres => sqlx::migrate!("db/migrations/postgres"),
            DatabaseKind::Sqlite => sqlx::migrate!("db/migrations/sqlite"),
        }
        .run(&mut conn)
        .await?;
        Ok(())
    }
}

#[derive(sqlx::FromRow)]
pub struct CheckResult(i64);

#[async_trait]
impl health::Checkable for DatabaseClient {
    type Error = DatabaseError;

    async fn check(&self) -> Result<(), Self::Error> {
        match sqlx::query_as::<Any, CheckResult>("SELECT $1")
            .bind(150_i64)
            .fetch_one(&self.pool)
            .await
        {
            Ok(CheckResult(150)) => Ok(()),
            Ok(_) => Err(Self::Error::HealthCheckFailure()),
            Err(e) => Err(Self::Error::Sqlx(Box::new(e))),
        }
    }

    fn name(&self) -> std::borrow::Cow<str> {
        std::borrow::Cow::Borrowed("database")
    }
}

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("sqlx error")]
    Sqlx(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("health check failure")]
    HealthCheckFailure(),
    #[error("invalid database_url")]
    ParseDatabaseUrlError(),
}
