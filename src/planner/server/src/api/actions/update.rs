use actix_web::HttpResponse;
use actix_web::web::Json;
use core::api::actions::update::update;
use core::structs::WorkItem;
use shared::auth::{TokenHeader, validate_token};
use shared::errors::ServiceError;

pub async fn update_work_item(
    token: TokenHeader,
    body: Json<WorkItem>,
) -> Result<HttpResponse, ServiceError> {
    validate_token(&token.value)?;
    Ok(HttpResponse::Accepted().json(update(body.into_inner()).await?))
}
