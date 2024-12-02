use axum::Router;

mod api;
mod models;
mod resources;
mod server;
mod web;

pub async fn run() -> color_eyre::Result<Router> {
    server::run().await
}
