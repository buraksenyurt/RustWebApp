use crate::structs::WorkItem;
use dal::json_repository::{save_all, select_all};
use shared::errors::{ServiceError, ServiceErrorStatus};
pub async fn update(work_item: WorkItem) -> Result<WorkItem, ServiceError> {
    let mut work_items = select_all::<WorkItem>()?;
    if !work_items.contains_key(&work_item.title) {
        return Err(ServiceError::new(
            ServiceErrorStatus::NotFound,
            format!("The work item with title '{}' not found", work_item.title),
        ));
    }
    work_items.insert(work_item.title.clone(), work_item.clone());
    save_all(&work_items).await?;
    Ok(work_item)
}
