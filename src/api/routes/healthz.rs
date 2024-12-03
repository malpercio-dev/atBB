//! Health checking endpoints.

use axum::{routing::get, Router};

mod liveness;
mod readiness;

pub fn compose() -> Router {
    Router::new()
        .route("/liveness", get(liveness::liveness))
        .route("/readiness", get(readiness::readiness))
}
