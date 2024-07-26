use std::time::Duration;

use actix_files as fs;
use actix_web::{middleware::{Compress, Logger}, rt::time::sleep, web, App, HttpServer, Responder};
use askama_actix::Template;
use tracing::level_filters::LevelFilter;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::EnvFilter;

#[derive(Template)]
#[template(path = "components/componentA/index.html")]
struct ComponentATemplate;

#[derive(Template)]
#[template(path = "components/componentB/index.html")]
struct ComponentBTemplate;

async fn component_a() -> impl Responder {
    ComponentATemplate
}

async fn component_b() -> impl Responder {
    sleep(Duration::from_secs(5)).await;
    ComponentBTemplate
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .pretty()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .wrap(TracingLogger::default())
            .wrap(Compress::default())
            .route("/componentA", web::get().to(component_a))
            .route("/componentB", web::get().to(component_b))
            .service(fs::Files::new("/", "./dist").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
