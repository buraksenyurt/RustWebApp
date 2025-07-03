use actix_web::HttpResponse;
use actix_web::web::Json;
use core::api::actions::update::update;
use core::structs::WorkItem;
use shared::errors::ServiceError;

pub async fn update_work_item(body: Json<WorkItem>) -> Result<HttpResponse, ServiceError> {
    Ok(HttpResponse::Accepted().json(update(body.into_inner()).await?))
}
