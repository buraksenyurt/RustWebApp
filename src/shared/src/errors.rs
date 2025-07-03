#[cfg(feature = "actix")]
use actix_web::{HttpResponse, error::ResponseError, http::StatusCode};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize, PartialEq)]
pub enum ServiceErrorStatus {
    #[error("Requested resource not found")]
    NotFound,
    #[error("Bad Request")]
    BadRequest,
    #[error("Internal Server Error")]
    InternalServerError,
    #[error("Unauthorized access")]
    Unauthorized,
    #[error("Conflict")]
    Conflict,
    #[error("Unknown error")]
    Unknown,
}

#[derive(Error, Debug, Serialize, Deserialize)]
pub struct ServiceError {
    pub status: ServiceErrorStatus,
    pub message: String,
}

impl ServiceError {
    pub fn new(status: ServiceErrorStatus, message: String) -> ServiceError {
        Self { status, message }
    }
}

impl Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[cfg(feature = "actix")]
impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        let status_code = match &self.status {
            ServiceErrorStatus::NotFound => StatusCode::NOT_FOUND,
            ServiceErrorStatus::BadRequest => StatusCode::BAD_REQUEST,
            ServiceErrorStatus::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceErrorStatus::Unauthorized => StatusCode::UNAUTHORIZED,
            ServiceErrorStatus::Unknown => StatusCode::INTERNAL_SERVER_ERROR,
            ServiceErrorStatus::Conflict => StatusCode::CONFLICT,
        };
        HttpResponse::build(status_code).json(self.message.clone())
    }
}
