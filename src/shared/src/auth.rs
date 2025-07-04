#[cfg(feature = "actix")]
use crate::errors::{ServiceError, ServiceErrorStatus};
#[cfg(feature = "actix")]
use actix_web::{FromRequest, HttpRequest, dev::Payload};
#[cfg(feature = "actix")]
use futures::future::{Ready, err, ok};

pub struct TokenHeader {
    pub value: String,
}

impl TokenHeader {
    pub fn new(value: String) -> Self {
        TokenHeader { value }
    }
}

#[cfg(feature = "actix")]
impl FromRequest for TokenHeader {
    type Error = ServiceError;
    type Future = Ready<Result<TokenHeader, ServiceError>>;
    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let raw = match req.headers().get("token") {
            Some(data) => data,
            None => {
                return err(ServiceError::new(
                    ServiceErrorStatus::Unauthorized,
                    "token not in header under key".to_string(),
                ));
            }
        };

        let token_value = match raw.to_str() {
            Ok(token) => token,
            Err(_) => {
                return err(ServiceError::new(
                    ServiceErrorStatus::Unauthorized,
                    "Invalid token".to_string(),
                ));
            }
        };
        return ok(TokenHeader {
            value: token_value.to_string(),
        });
    }
}

//todo@buraksenyurt Give this check another authority tool
pub fn validate_token(token: &str) -> Result<(), ServiceError> {
    if token == "APPLICATION_TOKEN_1234" {
        Ok(())
    } else {
        Err(ServiceError::new(
            ServiceErrorStatus::Unauthorized,
            "Invalid token provided".to_string(),
        ))
    }
}
