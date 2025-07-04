mod api;

use crate::api::actions::{configure_routes, create, delete, get, update};
use actix_web::web::{delete, get, post, put};
use actix_web::{App, HttpServer};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().expect("Failed to load .env file");
    println!("Server started at http://localhost:3000");
    HttpServer::new(|| App::new().configure(configure_routes))
        .workers(8)
        .bind("127.0.0.1:3000")?
        .run()
        .await
}
