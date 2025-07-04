use actix_web::{HttpResponse, web};
use core::{api::actions::create::create, structs::WorkItem};
use shared::{
    auth::{TokenHeader, validate_token},
    errors::ServiceError,
};

pub async fn create_work_item(
    token: TokenHeader,
    body: web::Json<WorkItem>,
) -> Result<HttpResponse, ServiceError> {
    validate_token(&token.value)?;
    Ok(HttpResponse::Ok().json(create(body.into_inner()).await?))
}
