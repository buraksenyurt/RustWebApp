use actix_web::HttpResponse;
use core::api::actions::get::*;

pub async fn fetch_all() -> HttpResponse {
    let result = match get_all().await {
        Ok(work_items) => work_items,
        Err(e) => return HttpResponse::InternalServerError().json(e),
    };
    HttpResponse::Ok().json(result)
}

pub async fn fetch_all_by_status(status: &str) -> HttpResponse {
    let result = match get_all_by_status(status).await {
        Ok(work_items) => work_items,
        Err(e) => return HttpResponse::InternalServerError().json(e),
    };
    HttpResponse::Ok().json(result)
}

pub async fn fetch_by_id(title: &str) -> HttpResponse {
    let result = match get_by_key(title).await {
        Ok(work_item) => work_item,
        Err(e) => return HttpResponse::InternalServerError().json(e),
    };
    HttpResponse::Ok().json(result)
}
