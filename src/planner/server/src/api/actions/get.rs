use actix_web::{HttpRequest, HttpResponse};
use core::api::actions::get::*;
use shared::auth::{TokenHeader, validate_token};
use shared::errors::{ServiceError, ServiceErrorStatus};
use std::str::FromStr;

pub async fn fetch_all(token: TokenHeader) -> Result<HttpResponse, ServiceError> {
    validate_token(&token.value)?;
    Ok(HttpResponse::Ok().json(get_all().await?))
}

pub async fn fetch_all_by_status(
    token: TokenHeader,
    request: HttpRequest,
) -> Result<HttpResponse, ServiceError> {
    validate_token(&token.value)?;
    let status = match request.match_info().get("status") {
        Some(s) => s,
        None => {
            return Err(ServiceError::new(
                ServiceErrorStatus::BadRequest,
                "Status not provided".to_string(),
            ));
        }
    };
    Ok(HttpResponse::Ok().json(get_all_by_status(status).await?))
}

pub async fn fetch_all_by_size_grater_than(
    token: TokenHeader,
    request: HttpRequest,
) -> Result<HttpResponse, ServiceError> {
    validate_token(&token.value)?;
    let size = match request.match_info().get("size") {
        Some(s) => s,
        None => {
            return Err(ServiceError::new(
                ServiceErrorStatus::BadRequest,
                "Size not provided".to_string(),
            ));
        }
    };
    let size = u8::from_str(size)
        .map_err(|e| ServiceError::new(ServiceErrorStatus::InternalServerError, e.to_string()))?;
    Ok(HttpResponse::Ok().json(get_all_by_size_grater_than(size).await?))
}

pub async fn fetch_by_id(
    token: TokenHeader,
    request: HttpRequest,
) -> Result<HttpResponse, ServiceError> {
    validate_token(&token.value)?;
    let title = match request.match_info().get("title") {
        Some(s) => s,
        None => {
            return Err(ServiceError::new(
                ServiceErrorStatus::BadRequest,
                "Title not provided".to_string(),
            ));
        }
    };
    Ok(HttpResponse::Ok().json(get_by_key(title).await?))
}
