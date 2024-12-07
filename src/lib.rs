use axum::Router;

mod api;
mod models;
mod resources;
mod server;
mod web;

pub use resources::DatabaseKind;

pub async fn run(config: Config) -> color_eyre::Result<Router> {
    server::run(config).await
}

/// Configuration for the application.
pub struct Config {
    /// The kind of database enginer to use.
    /// Can be POSTGRES or SQLITE
    pub database_kind: DatabaseKind,
}
