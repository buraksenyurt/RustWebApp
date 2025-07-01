use crate::structs::{WorkItem, WorkItems};
use dal::json_repository::{select_all, select_by_id};

pub async fn get_all() -> Result<WorkItems, String> {
    Ok(WorkItems::from(select_all::<WorkItem>()?))
}

pub async fn get_by_key(title: &str) -> Result<WorkItem, String> {
    Ok(WorkItem::from(select_by_id(title)?))
}

pub async fn get_all_by_status(status: &str) -> Result<WorkItems, String> {
    let all_items = select_all::<WorkItem>()?;

    let filtered = all_items
        .into_iter()
        .filter(|(_, item)| item.status.to_string().eq_ignore_ascii_case(status))
        .map(|(k, v)| (k, v))
        .collect();

    Ok(WorkItems::from(filtered))
}
