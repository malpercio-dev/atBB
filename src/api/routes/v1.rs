//! API v1 routes

use axum::Router;

use crate::api::ApiState;

mod forums;

pub fn compose(app_state: ApiState) -> Router {
    Router::new().nest("/forums", forums::compose(app_state))
}
