mod api;

use crate::api::actions::get::fetch_all;
use actix_web::{App, HttpRequest, HttpServer, Responder, web};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok().expect("Failed to load .env file");
    println!("Server started at http://localhost:3000");
    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .workers(8)
        .bind("127.0.0.1:3000")?
        .run()
        .await
}

async fn index(_request: HttpRequest) -> impl Responder {
    fetch_all().await
}
