use super::super::enums::WorkItemStatus;
use super::base::Base;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Ready {
    pub base: Base,
}

impl Ready {
    pub fn new(title: &str, size: u8) -> Self {
        Ready {
            base: Base {
                title: title.to_string(),
                size,
                status: WorkItemStatus::Ready,
            },
        }
    }
}
