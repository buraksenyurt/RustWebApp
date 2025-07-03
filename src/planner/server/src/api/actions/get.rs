use actix_web::HttpResponse;
use core::api::actions::get::*;
use shared::errors::ServiceError;

pub async fn fetch_all() -> Result<HttpResponse, ServiceError> {
    Ok(HttpResponse::Ok().json(get_all().await?))
}

pub async fn fetch_all_by_status(status: &str) -> Result<HttpResponse, ServiceError> {
    Ok(HttpResponse::Ok().json(get_all_by_status(status).await?))
}

pub async fn fetch_all_by_size_grater_than(size: u8) -> Result<HttpResponse, ServiceError> {
    Ok(HttpResponse::Ok().json(get_all_by_size_grater_than(size).await?))
}

pub async fn fetch_by_id(title: &str) -> Result<HttpResponse, ServiceError> {
    Ok(HttpResponse::Ok().json(get_by_key(title).await?))
}
