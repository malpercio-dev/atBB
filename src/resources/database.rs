use async_trait::async_trait;
use sea_query::{PostgresQueryBuilder, QueryBuilder, SchemaBuilder, SqliteQueryBuilder};
use sqlx::{any::install_default_drivers, Any, AnyPool, Pool};
use sqlx_migrator::{Info, Migrate, Migrator, Plan};
use thiserror::Error;

mod migrations;

#[derive(PartialEq)]
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
        match s {
            "SQLITE" | "sqlite" => Ok(Self::Sqlite),
            "POSTGRES" | "postgres" => Ok(Self::Postgres),
            _ => Err(DatabaseKindParseError("could not parse DatabaseKind")),
        }
    }
}

/// A client for interfacing with a database.
pub struct DatabaseClient {
    pool: Pool<Any>,
}

impl DatabaseClient {
    pub async fn new(database_kind: DatabaseKind) -> color_eyre::Result<Self> {
        let (url, _box_query_builder, _box_schema_builder): (
            &str,
            Box<dyn QueryBuilder>,
            Box<dyn SchemaBuilder>,
        ) = if database_kind == DatabaseKind::Postgres {
            (
                "postgres://sea:sea@127.0.0.1/query",
                Box::new(PostgresQueryBuilder {}),
                Box::new(PostgresQueryBuilder {}),
            )
        } else if database_kind == DatabaseKind::Sqlite {
            (
                "sqlite::memory:",
                Box::new(SqliteQueryBuilder {}),
                Box::new(SqliteQueryBuilder {}),
            )
        } else {
            panic!()
        };

        install_default_drivers();
        let pool = AnyPool::connect(url).await.unwrap();
        Self::migrate(&pool).await?;
        Ok(Self { pool })
    }

    pub async fn migrate(pool: &Pool<Any>) -> color_eyre::Result<(), sqlx_migrator::Error> {
        let mut migrator = Migrator::default();
        migrator.add_migrations(migrations::migrations());
        let mut conn = pool.acquire().await?;
        migrator.run(&mut *conn, &Plan::apply_all()).await
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
}
