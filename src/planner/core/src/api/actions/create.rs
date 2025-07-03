use crate::structs::WorkItem;
use dal::json_repository::save_single;
use shared::errors::ServiceError;

pub fn create(work_item: WorkItem) -> Result<WorkItem, ServiceError> {
    save_single(&work_item.title.to_string(), &work_item)?;
    Ok(work_item)
}

#[cfg(test)]
mod tests {
    use crate::api::actions::create::create;
    use crate::enums::WorkItemStatus;
    use crate::structs::WorkItem;

    #[test]
    fn create_action_in_ready_state_test() {
        let work_item = WorkItem {
            title: "Read a book".to_string(),
            size: 5,
            status: WorkItemStatus::Ready,
        };
        let created = create(work_item);
        assert!(created.is_ok());
        assert_eq!(created.unwrap().title, "Read a book".to_string());
    }
}
