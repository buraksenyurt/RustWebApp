use crate::enums::WorkItemStatus;
use crate::structs::WorkItem;
use dal::json_repository::save_single;

pub fn create(title: &str, size: u8, status: WorkItemStatus) -> Result<WorkItem, String> {
    let work_item = WorkItem {
        title: title.to_string(),
        size,
        status,
    };
    save_single(title, &work_item)?;
    Ok(work_item)
}

#[cfg(test)]
mod tests {
    use crate::api::actions::create::create;
    use crate::enums::WorkItemStatus;

    #[test]
    fn create_action_in_ready_state_test() {
        let created = create("Read a book", 5, WorkItemStatus::Ready);
        assert!(created.is_ok());
        assert_eq!(created.unwrap().title, "Read a book".to_string());
    }
}
