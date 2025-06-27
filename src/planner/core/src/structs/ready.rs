use super::super::enums::WorkItemStatus;
use super::base::Base;

pub struct Ready {
    pub base: Base,
}

impl Ready {
    pub fn new(title: String, size: u8) -> Self {
        Ready {
            base: Base {
                title,
                size,
                status: WorkItemStatus::Ready,
            },
        }
    }
}
