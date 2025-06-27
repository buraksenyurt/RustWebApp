use super::super::enums::WorkItemStatus;
use super::base::Base;

pub struct Completed {
    pub base: Base,
}

impl Completed {
    pub fn new(title: String, size: u8) -> Self {
        Completed {
            base: Base {
                title,
                size,
                status: WorkItemStatus::Completed,
            },
        }
    }
}
