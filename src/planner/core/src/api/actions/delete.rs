use crate::structs::WorkItem;
use dal::json_repository::delete_single;
use shared::errors::ServiceError;

pub async fn delete(key: &str) -> Result<(), ServiceError> {
    delete_single::<WorkItem>(key).await
}
