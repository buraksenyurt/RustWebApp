use super::super::enums::WorkItemStatus;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Base {
    pub title: String,
    pub size: u8,
    pub status: WorkItemStatus,
}
