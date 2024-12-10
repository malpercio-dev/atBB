use axum::Router;

mod api;
mod models;
mod resources;
mod server;
mod web;

pub use resources::DatabaseKind;
pub use resources::DatabaseUrl;

/// Configuration for the application.
pub struct Config {
    /// The kind of database engine to use.
    /// Can be Postgres or Sqlite
    pub database_kind: DatabaseKind,

    /// URL for the database.
    /// For Sqlite, can be something like sqlite::memory:, sqlite:data.db
    /// For Postgres, postgresql://localhost/mydb, postgresql://user@localhost
    pub database_url: DatabaseUrl,
}

pub async fn run(config: Config) -> color_eyre::Result<Router> {
    server::run(config).await
}
