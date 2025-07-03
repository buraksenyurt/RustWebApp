use crate::structs::{WorkItem, WorkItems};
use dal::json_repository::{select_all, select_by_id};
use shared::errors::ServiceError;

pub async fn get_all() -> Result<WorkItems, ServiceError> {
    Ok(WorkItems::from(select_all::<WorkItem>()?))
}

pub async fn get_by_key(title: &str) -> Result<WorkItem, ServiceError> {
    Ok(WorkItem::from(select_by_id(title)?))
}

pub async fn get_all_by_status(status: &str) -> Result<Vec<WorkItem>, ServiceError> {
    let all_items = select_all::<WorkItem>()?;

    let filtered = all_items
        .into_iter()
        .filter(|(_, item)| item.status.to_string().eq_ignore_ascii_case(status))
        .map(|(_, v)| v)
        .collect();

    Ok(filtered)
}

pub async fn get_all_by_size_grater_than(size: u8) -> Result<Vec<WorkItem>, ServiceError> {
    let all_items = select_all::<WorkItem>()?;

    let filtered = all_items
        .into_iter()
        .filter(|(_, item)| item.size >= size)
        .map(|(_, v)| v)
        .collect();

    Ok(filtered)
}
