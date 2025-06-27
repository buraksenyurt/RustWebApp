use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum WorkItemStatus {
    Ready,
    Completed,
    InProgress,
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

    #[test]
    fn work_item_status_convert_to_right_string_test() {
        let status = WorkItemStatus::Ready;
        assert_eq!(status.to_string(), "Ready");

        let status = WorkItemStatus::InProgress;
        assert_eq!(status.to_string(), "InProgress");

        let status = WorkItemStatus::Completed;
        assert_eq!(status.to_string(), "Completed");
    }
}
