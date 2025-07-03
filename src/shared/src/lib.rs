use crate::errors::{ServiceError, ServiceErrorStatus};

pub mod errors;

#[allow(dead_code)]
fn simulate_get_categories(category_id: u32) -> Result<String, ServiceError> {
    if category_id == 0 {
        Err(ServiceError::new(
            ServiceErrorStatus::BadRequest,
            "categories cannot be 0".to_string(),
        ))
    } else if category_id == 404 {
        Err(ServiceError::new(
            ServiceErrorStatus::NotFound,
            "Category 404 not found".to_string(),
        ))
    } else {
        Ok(format!("[{}]Category list...", category_id))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::{ServiceError, ServiceErrorStatus};

    #[test]
    fn simulate_get_categories_returns_error_test() {
        let result = simulate_get_categories(0);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.status, ServiceErrorStatus::BadRequest);
        assert_eq!(error.message, "categories cannot be 0".to_string());
    }

    #[test]
    fn simulate_get_categories_returns_not_found_error_test() {
        let result = simulate_get_categories(404);
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert_eq!(error.status, ServiceErrorStatus::NotFound);
        assert_eq!(error.message, "Category 404 not found".to_string());
    }

    #[test]
    fn simulate_get_categories_returns_correct_test() {
        let result = simulate_get_categories(12);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), String::from("[12]Category list..."));
    }

    #[test]
    fn service_error_new_test() {
        let error = ServiceError::new(
            ServiceErrorStatus::BadRequest,
            "Calling function is not operational".to_string(),
        );
        assert_eq!(error.status, ServiceErrorStatus::BadRequest);
        assert_eq!(
            error.message,
            "Calling function is not operational".to_string()
        );
    }

    #[test]
    fn service_error_display_test() {
        let error = ServiceError::new(
            ServiceErrorStatus::BadRequest,
            String::from("The resource path not found"),
        );
        assert_eq!(
            format!("{}", error),
            String::from("The resource path not found")
        );
    }
}

#[cfg(all(test, feature = "actix"))]
mod actix_tests {
    use super::*;
    use actix_web::ResponseError;
    use actix_web::http::StatusCode;

    #[test]
    fn service_error_response_returns_correct_status() {
        let error = ServiceError::new(
            ServiceErrorStatus::Unauthorized,
            "Access denied".to_string(),
        );
        let response = error.error_response();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }
}
