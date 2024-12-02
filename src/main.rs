// use actix_web::{get, web, HttpServer, Responder};
// use color_eyre::eyre::Result;
// use lazy_static::lazy_static;
// use serde::{Deserialize, Serialize};
// use tera::Tera;

// #[get("/forums")]
// async fn forums_handler(
//     _action: actix_web::web::Path<(String, String)>,
//     _state: actix_web::web::Data<AppState>,
// ) -> impl Responder {
//     let ctx = tera::Context::new();

//     let rendered = TEMPLATES.render("components/forums.html", &ctx).unwrap();

//     actix_web::HttpResponse::Ok().body(rendered)
// }

// }

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    let router = atbb::run().await?;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, router).await.unwrap();
    Ok(())
}
