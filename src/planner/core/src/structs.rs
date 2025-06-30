use super::enums::WorkItemStatus;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct WorkItems {
    pub ready: Vec<WorkItem>,
    pub in_progress: Vec<WorkItem>,
    pub completed: Vec<WorkItem>,
}

impl WorkItems {
    pub fn from(data_list: HashMap<String, WorkItem>) -> WorkItems {
        let mut ready = Vec::new();
        let mut in_progress = Vec::new();
        let mut completed = Vec::new();

        for (_, item) in data_list {
            match item.status {
                WorkItemStatus::Ready => {
                    ready.push(item);
                }
                WorkItemStatus::InProgress => {
                    in_progress.push(item);
                }
                WorkItemStatus::Completed => {
                    completed.push(item);
                }
            }
        }

        WorkItems {
            ready,
            in_progress,
            completed,
        }
    }
}
