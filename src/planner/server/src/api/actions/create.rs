use actix_web::HttpResponse;
use core::api::actions::create::create;
use core::structs::WorkItem;

pub async fn add(work_item: WorkItem) -> HttpResponse {
    let work_item = match create(work_item) {
        Ok(item) => item,
        Err(e) => return HttpResponse::InternalServerError().json(e),
    };
    HttpResponse::Ok().json(work_item)
}
