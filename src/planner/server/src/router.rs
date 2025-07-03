use crate::api::actions::create::add;
use crate::api::actions::get::*;
use actix_web::{HttpRequest, HttpResponse, web};
use core::structs::WorkItem;
use shared::errors::{ServiceError, ServiceErrorStatus};
use std::str::FromStr;

pub async fn get_all_work_items() -> Result<HttpResponse, ServiceError> {
    fetch_all().await
}

pub async fn get_all_work_items_by_status(
    request: HttpRequest,
) -> Result<HttpResponse, ServiceError> {
    let status = match request.match_info().get("status") {
        Some(s) => s,
        None => {
            return Err(ServiceError::new(
                ServiceErrorStatus::BadRequest,
                "Status not provided".to_string(),
            ));
        }
    };
    fetch_all_by_status(status).await
}

pub async fn get_all_work_items_by_size_grater_than(
    request: HttpRequest,
) -> Result<HttpResponse, ServiceError> {
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

    fetch_all_by_size_grater_than(size).await
}

pub async fn get_work_item_by_id(request: HttpRequest) -> Result<HttpResponse, ServiceError> {
    let title = match request.match_info().get("title") {
        Some(s) => s,
        None => {
            return Err(ServiceError::new(
                ServiceErrorStatus::BadRequest,
                "Title not provided".to_string(),
            ));
        }
    };
    fetch_by_id(title).await
}

pub async fn create_work_item(body: web::Json<WorkItem>) -> Result<HttpResponse, ServiceError> {
    add(body.into_inner()).await
}
