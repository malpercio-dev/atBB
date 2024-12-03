use axum::{extract::State, response::Html};

use crate::models::forum::Forum;

use super::TEMPLATES;

pub async fn index(State(forums): State<Vec<Forum>>) -> Html<String> {
    let mut ctx = tera::Context::new();
    ctx.insert("forums", &forums);

    let rendered = TEMPLATES.render("index.html", &ctx).unwrap();

    Html(rendered)
}
