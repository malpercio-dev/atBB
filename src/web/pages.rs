//! Routes to serve HTML pages

use axum::{routing::get, Router};
use lazy_static::lazy_static;
use tera::Tera;
use tower_http::services::ServeDir;

use crate::web::WebState;

mod index;

pub fn compose(app_state: WebState) -> Router {
    Router::new()
        .route("/", get(index::index))
        .with_state(app_state)
        .nest_service("/static", ServeDir::new("src/web/static"))
}

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        match Tera::new("src/web/templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Template parsing error(s): {e}");
                ::std::process::exit(1);
            }
        }
    };
}
