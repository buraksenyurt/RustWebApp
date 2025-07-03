use actix_web::{HttpRequest, HttpResponse};
use core::api::actions::delete::delete;
use shared::errors::{ServiceError, ServiceErrorStatus};
pub async fn delete_by_title(request: HttpRequest) -> Result<HttpResponse, ServiceError> {
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
