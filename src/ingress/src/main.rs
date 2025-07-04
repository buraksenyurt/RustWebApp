use actix_web::{App, HttpRequest, HttpResponse, HttpServer, Responder, web};
use rust_embed::RustEmbed;
use server::api::configure_routes;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Server is running on: localhost:8001");
    HttpServer::new(|| {
        App::new()
            .configure(configure_routes)
            .default_service(web::route().to(handle_all))
    })
    .bind("0.0.0.0:8001")?
    .run()
    .await
}

async fn index() -> HttpResponse {
    let html = Assets::get("index.html").expect("index.html not found");
    HttpResponse::Ok().content_type("text/html").body(html.data)
}

#[derive(RustEmbed)]
#[folder = "./web/dist"]
struct Assets;

fn serve_asset(path: String) -> HttpResponse {
    let path = path.trim_start_matches('/');

    match Assets::get(path) {
        Some(content) => {
            let mime = mime_guess::from_path(path).first_or_octet_stream();
            HttpResponse::Ok()
                .content_type(mime.as_ref())
                .append_header(("Cache-Control", "public, max-age=300"))
                .body(content.data)
        }
        None => HttpResponse::NotFound().finish(),
    }
}
async fn handle_all(request: HttpRequest) -> impl Responder {
    if request.path().contains("/api/") {
        return HttpResponse::NotFound().finish();
    }
    if request.path().contains("web/public") {
        return serve_asset(request.path().to_string());
    }
    let file_type = mime_guess::from_path(&request.path())
        .first_raw()
        .unwrap_or_else(|| "text/html");

    if !file_type.contains("text/html") {
        return serve_asset(request.path().to_string());
    }
    index().await
}
