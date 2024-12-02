use actix_web::{get, web, HttpServer, Responder};
use color_eyre::eyre::Result;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use tera::Tera;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        match Tera::new("src/templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Template parsing error(s): {e}");
                ::std::process::exit(1);
            }
        }
    };
}

#[derive(Serialize)]
struct AppState {
    forums: Vec<Forum>,
}

impl AppState {
    fn get_forums(&self) -> &Vec<Forum> {
        &self.forums
    }
}

#[derive(Serialize, Deserialize)]
struct Forum {
    id: &'static str,
    name: &'static str,
    description: &'static str,
}

#[get("/")]
async fn index(state: actix_web::web::Data<AppState>) -> impl Responder {
    let mut ctx = tera::Context::new();
    ctx.insert("forums", state.get_forums());

    let rendered = TEMPLATES.render("index.html", &ctx).unwrap();

    actix_web::HttpResponse::Ok().body(rendered)
}

#[get("/forums")]
async fn forums_handler(
    _action: actix_web::web::Path<(String, String)>,
    _state: actix_web::web::Data<AppState>,
) -> impl Responder {
    let ctx = tera::Context::new();

    let rendered = TEMPLATES.render("components/forums.html", &ctx).unwrap();

    actix_web::HttpResponse::Ok().body(rendered)
}

#[actix_web::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    HttpServer::new(|| {
        actix_web::App::new()
            // register state
            .app_data(web::Data::new(AppState {
                forums: vec![Forum {
                    id: "asdf",
                    name: "hello world",
                    description: "this is a forum",
                }],
            }))
            // index route
            .service(index)
            // forums route
            .service(forums_handler)
            // static files
            .service(actix_files::Files::new("/", "./src/static/").show_files_listing())
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;

    Ok(())
}
