use crate::enums::WorkItemStatus;
use crate::structs::{completed::Completed, in_progress::InProgress, ready::Ready};
use std::fmt::{Display, Formatter};

pub fn create(title: &str, size: u8, status: WorkItemStatus) -> ActionTypes {
    match status {
        WorkItemStatus::Ready => ActionTypes::Ready(Ready::new(&title, size)),
        WorkItemStatus::Completed => ActionTypes::Completed(Completed::new(title, size)),
        WorkItemStatus::InProgress => ActionTypes::InProgress(InProgress::new(title, size)),
    }
}

pub enum ActionTypes {
    Ready(Ready),
    InProgress(InProgress),
    Completed(Completed),
}

impl Display for ActionTypes {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ActionTypes::Ready(ready) => {
                write!(f, "Ready -> '{}'({})", ready.base.title, ready.base.size)
            }
            ActionTypes::InProgress(in_progress) => {
                write!(
                    f,
                    "Progressing -> '{}'({})",
                    in_progress.base.title, in_progress.base.size
                )
            }
            ActionTypes::Completed(completed) => {
                write!(
                    f,
                    "Completed -> '{}'({})",
                    completed.base.title, completed.base.size
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::api::actions::create::create;
    use crate::enums::WorkItemStatus;

    #[test]
    fn create_action_in_ready_state_test() {
        let action_type = create("Read a book", 5, WorkItemStatus::Ready);
        assert_eq!(action_type.to_string(), "Ready -> 'Read a book'(5)");
    }
}
