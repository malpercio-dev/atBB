//! The application's API routes.

use axum::Router;

use crate::api::ApiState;

mod healthz;
mod v1;

pub fn compose(app_state: ApiState) -> Router {
    Router::new()
        .nest("/healthz", healthz::compose())
        .nest("/v1", v1::compose(app_state))
}
