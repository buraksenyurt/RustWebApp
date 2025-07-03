mod api;
mod router;

use crate::router::*;
use actix_web::{App, HttpServer, web};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().expect("Failed to load .env file");
    println!("Server started at http://localhost:3000");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_all_work_items))
            .route(
                "filter/by/status/{status}",
                web::get().to(get_all_work_items_by_status),
            )
            .route(
                "/filter/by/size/{size}",
                web::get().to(get_all_work_items_by_size_grater_than),
            )
            .route("/{title}", web::get().to(get_work_item_by_id))
            .route("/", web::post().to(create_work_item))
    })
    .workers(8)
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
