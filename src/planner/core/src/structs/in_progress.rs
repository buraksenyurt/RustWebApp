use super::super::enums::WorkItemStatus;
use super::base::Base;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct InProgress {
    pub base: Base,
}

impl InProgress {
    pub fn new(title: &str, size: u8) -> Self {
        InProgress {
            base: Base {
                title: title.to_string(),
                size,
                status: WorkItemStatus::InProgress,
            },
        }
    }
}
