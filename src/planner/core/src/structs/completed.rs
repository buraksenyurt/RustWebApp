use super::super::enums::WorkItemStatus;
use super::base::Base;

pub struct Completed {
    pub base: Base,
}

impl Completed {
    pub fn new(title: &str, size: u8) -> Self {
        Completed {
            base: Base {
                title: title.to_string(),
                size,
                status: WorkItemStatus::Completed,
            },
        }
    }
}
