mod api;

use crate::api::configure_routes;
use actix_cors::Cors;
use actix_web::{App, HttpServer};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().expect("Failed to load .env file");
    println!("Server started at http://localhost:3000");
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new().wrap(cors).configure(configure_routes)
    })
    .workers(8)
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
