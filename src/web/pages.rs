//! Routes to serve HTML pages

use axum::{routing::get, Router};
use axum_embed::ServeEmbed;
use lazy_static::lazy_static;
use rust_embed::Embed;
use tera::Tera;

use crate::web::WebState;

mod index;

#[derive(Embed, Clone)]
#[folder = "src/web/static"]
#[include = "*.css"]
struct StaticAssets;

#[derive(Embed)]
#[folder = "src/web/templates"]
#[include = "*.html"]
struct Templates;

pub fn compose(app_state: WebState) -> Router {
    Router::new()
        .route("/", get(index::index))
        .with_state(app_state)
        .nest_service("/static", ServeEmbed::<StaticAssets>::new())
}

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = Tera::default();
        for path in Templates::iter() {
            let file = Templates::get(&path).unwrap();
            let contents = match std::str::from_utf8(file.data.as_ref()) {
                Ok(c) => c,
                Err(e) => {
                    println!("Template parsing error(s): {e}");
                    panic!("Template parsing error(s): {e}");
                }
            };
            match tera.add_raw_template(&path, contents) {
                Ok(t) => t,
                Err(e) => {
                    println!("Template parsing error(s): {e}");
                    panic!("Template parsing error(s): {e}");
                }
            }
        }
        tera
    };
}
