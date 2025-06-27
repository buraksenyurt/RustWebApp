use super::super::enums::WorkItemStatus;
use super::base::Base;

pub struct InProgress {
    pub base: Base,
}

impl InProgress {
    pub fn new(title: String, size: u8) -> Self {
        InProgress {
            base: Base {
                title,
                size,
                status: WorkItemStatus::InProgress,
            },
        }
    }
}
