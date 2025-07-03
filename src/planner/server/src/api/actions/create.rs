use actix_web::{HttpResponse, web};
use core::api::actions::create::create;
use core::structs::WorkItem;
use shared::errors::ServiceError;

pub async fn create_work_item(body: web::Json<WorkItem>) -> Result<HttpResponse, ServiceError> {
    Ok(HttpResponse::Ok().json(create(body.into_inner()).await?))
}
