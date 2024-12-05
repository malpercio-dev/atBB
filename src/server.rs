//! The application's web server.
//! Serves both api and web traffic.

use crate::{
    api::{self, ApiState, ApiStateInner},
    models::forum::{Forum, ForumId},
    resources::DatabaseClient,
    web::{pages, WebState, WebStateInner},
};
use axum::{Extension, Router};

pub async fn run() -> color_eyre::Result<Router> {
    let database = DatabaseClient::new()?;
    let database_health_check =
        health::PeriodicChecker::new(database.clone(), health::Config::default());
    tokio::spawn(database_health_check.clone().run());

    let web_state = WebState {
        inner: WebStateInner {
            forums: vec![Forum {
                id: ForumId("asdf".into()),
                name: "hello world".to_string(),
                description: "this is a forum".to_string(),
            }],
        }
        .into(),
    };

    let api_state = ApiState {
        inner: ApiStateInner {
            forums: vec![Forum {
                id: ForumId("asdf".into()),
                name: "hello world".to_string(),
                description: "this is a forum".to_string(),
            }],
        }
        .into(),
    };

    let router = Router::new()
        .nest("/", pages::compose(web_state))
        .nest("/api/v1", api::routes::v1::compose(api_state))
        .nest("/healthz", api::routes::healthz::compose())
        .route_layer(Extension(database_health_check));
    Ok(router)
}
