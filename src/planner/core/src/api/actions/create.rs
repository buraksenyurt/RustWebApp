use crate::structs::WorkItem;
use dal::json_repository::save_single;
use shared::errors::ServiceError;

pub async fn create(work_item: WorkItem) -> Result<WorkItem, ServiceError> {
    save_single(&work_item.title.to_string(), &work_item).await?;
    Ok(work_item)
}
