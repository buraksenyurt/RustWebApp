use super::super::enums::WorkItemStatus;

pub struct Base {
    pub title: String,
    pub size: u8,
    pub status: WorkItemStatus,
}
