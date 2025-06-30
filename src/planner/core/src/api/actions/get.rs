use crate::structs::{WorkItem, WorkItems};
use dal::json_repository::select_all;

pub async fn get_all() -> Result<WorkItems, String> {
    Ok(WorkItems::from(select_all::<WorkItem>()?))
}
