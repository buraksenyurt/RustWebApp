use crate::structs::{WorkItem, WorkItems};
use dal::json_repository::{select_all, select_by_id};

pub async fn get_all() -> Result<WorkItems, String> {
    Ok(WorkItems::from(select_all::<WorkItem>()?))
}

pub async fn get_by_key(title: &str) -> Result<WorkItem, String> {
    Ok(WorkItem::from(select_by_id(title)?))
}
