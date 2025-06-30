use actix_web::HttpResponse;
use core::api::actions::get::get_all;

pub async fn fetch_all() -> HttpResponse {
    let work_items = match get_all().await {
        Ok(work_items) => work_items,
        Err(e) => return HttpResponse::InternalServerError().json(e),
    };
    HttpResponse::Ok().json(work_items)
}
