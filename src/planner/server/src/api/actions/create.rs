use actix_web::HttpResponse;
use core::api::actions::create::create;
use core::structs::WorkItem;
use shared::errors::ServiceError;

pub async fn add(work_item: WorkItem) -> Result<HttpResponse, ServiceError> {
    Ok(HttpResponse::Ok().json(create(work_item)?))
}
