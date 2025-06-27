use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum WorkItemStatus {
    Ready,
    Completed,
    InProgress,
}

impl FromStr for WorkItemStatus {
    type Err = String;
    fn from_str(status: &str) -> Result<WorkItemStatus, Self::Err> {
        match status.to_lowercase().as_str() {
            "ready" => Ok(WorkItemStatus::Ready),
            "inprogress" => Ok(WorkItemStatus::InProgress),
            "completed" => Ok(WorkItemStatus::Completed),
            _ => Err(format!("Unknown work item status: '{}'", status)),
        }
    }
}

impl Display for WorkItemStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            WorkItemStatus::Ready => f.write_str("Ready"),
            WorkItemStatus::Completed => write!(f, "Completed"),
            WorkItemStatus::InProgress => write!(f, "InProgress"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::enums::WorkItemStatus;
    use std::str::FromStr;

    #[test]
    fn work_item_status_convert_to_correct_string_test() {
        let status = WorkItemStatus::Ready;
        assert_eq!(status.to_string(), "Ready");

        let status = WorkItemStatus::InProgress;
        assert_eq!(status.to_string(), "InProgress");

        let status = WorkItemStatus::Completed;
        assert_eq!(status.to_string(), "Completed");
    }

    #[test]
    fn work_item_status_from_str_test() {
        let status = WorkItemStatus::from_str("Ready").unwrap();
        assert_eq!(status, WorkItemStatus::Ready);

        let status = WorkItemStatus::from_str("reADy").unwrap();
        assert_eq!(status, WorkItemStatus::Ready);

        let status = WorkItemStatus::from_str("INPROGRESS").unwrap();
        assert_eq!(status, WorkItemStatus::InProgress);

        let status = WorkItemStatus::from_str("inProgreSS").unwrap();
        assert_eq!(status, WorkItemStatus::InProgress);
    }
}
