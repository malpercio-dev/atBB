//! The application's web server.
//! Serves both api and web traffic.

use crate::{
    api::{self, ApiState, ApiStateInner},
    models::forum::{Forum, ForumId},
    resources::DatabaseClient,
    web::{pages, WebState, WebStateInner},
    Config,
};
use axum::{Extension, Router};

pub async fn run(config: Config) -> color_eyre::Result<Router> {
    let database = DatabaseClient::new(config.database_kind, config.database_url).await?;
    database.migrate().await?;
    let database_health_check = health::PeriodicChecker::new(database, health::Config::default());
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
        .nest("/api", api::routes::compose(api_state))
        .route_layer(Extension(database_health_check));
    Ok(router)
}
