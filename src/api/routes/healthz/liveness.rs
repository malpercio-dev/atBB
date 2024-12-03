//! Liveness probe for container orchestration platforms.
//! A successful response indicates that the application is live.

use axum::response::IntoResponse;

pub async fn liveness() -> impl IntoResponse {}
