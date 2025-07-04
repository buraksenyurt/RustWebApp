use actix_web::{HttpRequest, HttpResponse};
use core::api::actions::delete::delete;
use shared::auth::{TokenHeader, validate_token};
use shared::errors::{ServiceError, ServiceErrorStatus};
pub async fn delete_by_title(
    token: TokenHeader,
    request: HttpRequest,
) -> Result<HttpResponse, ServiceError> {
    validate_token(&token.value)?;
    match request.match_info().get("title") {
        Some(key) => {
            delete(key).await?;
        }
        None => {
            return Err(ServiceError::new(
                ServiceErrorStatus::BadRequest,
                "'title' not provided".to_string(),
            ));
        }
    };
    Ok(HttpResponse::Ok().finish())
}
