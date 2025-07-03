mod api;

use crate::api::actions::{create, delete, get, update};
use actix_web::web::{delete, get, post, put};
use actix_web::{App, HttpServer};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().expect("Failed to load .env file");
    println!("Server started at http://localhost:3000");
    HttpServer::new(|| {
        App::new()
            .route("/", get().to(get::fetch_all))
            .route(
                "filter/by/status/{status}",
                get().to(get::fetch_all_by_status),
            )
            .route(
                "/filter/by/size/{size}",
                get().to(get::fetch_all_by_size_grater_than),
            )
            .route("/{title}", get().to(get::fetch_by_id))
            .route("/", post().to(create::create_work_item))
            .route("/", put().to(update::update_work_item))
            .route("/{title}", delete().to(delete::delete_by_title))
    })
    .workers(8)
    .bind("127.0.0.1:3000")?
    .run()
    .await
}
