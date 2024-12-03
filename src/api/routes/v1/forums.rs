//! Forums API routes

use axum::{routing::get, Router};

use crate::api::ApiState;

mod get;

pub fn compose(app_state: ApiState) -> Router {
    Router::new()
        .route("/", get(get::get_forums))
        .with_state(app_state)
}
