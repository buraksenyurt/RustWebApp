use super::enums::WorkItemStatus;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct WorkItem {
    pub title: String,
    pub size: u8,
    pub status: WorkItemStatus,
}

impl Display for WorkItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.status {
            WorkItemStatus::Ready => write!(f, "Ready -> '{}({})'", self.title, self.size),
            WorkItemStatus::InProgress => {
                write!(f, "In Progress -> '{}({})'", self.title, self.size)
            }
            WorkItemStatus::Completed => write!(f, "Completed -> '{}({})'", self.title, self.size),
        }
    }
}
