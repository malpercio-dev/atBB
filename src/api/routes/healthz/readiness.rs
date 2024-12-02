//! Readiness probe for container orchestration platforms.
//! A successful response indicates that the application is ready
//! to serve traffic.
//!

use axum::{http, response::IntoResponse, Extension};
use health::Reporter;

use crate::resources::DatabaseClient;

pub async fn readiness(
    hc: Extension<health::PeriodicChecker<DatabaseClient>>,
) -> impl IntoResponse {
    let report = match hc.last_check() {
        health::Check::Pass => "OK",
        health::Check::Failed => "ERROR",
    };

    match hc.status() {
        Some(health::Status::Healthy) => (http::StatusCode::OK, report),
        Some(health::Status::Unhealthy) => (http::StatusCode::SERVICE_UNAVAILABLE, report),
        None => (http::StatusCode::SERVICE_UNAVAILABLE, "UNAVAILABLE"),
    }
}
