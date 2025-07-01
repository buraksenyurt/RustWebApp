mod api;

use crate::api::actions::create::add;
use crate::api::actions::get::{fetch_all, fetch_all_by_status, fetch_by_id};
use actix_web::{App, HttpRequest, HttpServer, Responder, web};
use core::structs::WorkItem;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok().expect("Failed to load .env file");
    println!("Server started at http://localhost:3000");
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_all_work_items))
            .route(
                "/filter/{status}",
                web::get().to(get_all_work_items_by_staus),
            )
            .route("/{title}", web::get().to(get_work_item_by_id))
            .route("/", web::post().to(create_work_item))
    })
    .workers(8)
    .bind("127.0.0.1:3000")?
    .run()
    .await
}

async fn get_all_work_items(_request: HttpRequest) -> impl Responder {
    fetch_all().await
}

async fn get_all_work_items_by_staus(path: web::Path<String>) -> impl Responder {
    let status = path.into_inner();
    fetch_all_by_status(&status).await
}

async fn get_work_item_by_id(path: web::Path<String>) -> impl Responder {
    let title = path.into_inner();
    fetch_by_id(&title).await
}

async fn create_work_item(body: web::Json<WorkItem>) -> impl Responder {
    let wi = body.into_inner();
    add(wi).await
}
