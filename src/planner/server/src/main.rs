use actix_web::{App, HttpRequest, HttpServer, Responder, web};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Server started at http://localhost:8080");
    HttpServer::new(|| App::new().route("/", web::get().to(index)))
        .workers(8)
        .bind("127.0.0.1:8080")?
        .run()
        .await
}

async fn index(_request: HttpRequest) -> impl Responder {
    "Planner Main Page"
}
