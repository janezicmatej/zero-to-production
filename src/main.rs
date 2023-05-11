use actix_web::{web, App, HttpRequest, HttpServer, Responder};

const VERSION: &str = env!("CARGO_PKG_VERSION");

async fn version(_: HttpRequest) -> impl Responder {
    VERSION
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://0.0.0.0:8000");

    HttpServer::new(|| App::new().route("/_version", web::get().to(version)))
        .bind("0.0.0.0:8000")?
        .run()
        .await
}
