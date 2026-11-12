use serde::{Deserialize, Serialize};

use super::enums::TaskStatus;
use glue::errors::NanoServiceError;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewToDoItem {
    pub name: String,
    pub status: TaskStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "sqlx-postgres", derive(sqlx::FromRow))]
pub struct ToDoItem {
    pub name: String,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AllTodoItems {
    pub pending: Vec<ToDoItem>,
    pub done: Vec<ToDoItem>,
}

impl AllTodoItems {
    pub fn from_vec(all_items: Vec<ToDoItem>) -> Result<Self, NanoServiceError> {
        let mut pending = Vec::new();
        let mut done = Vec::new();

        for item in all_items {
            match TaskStatus::from_string(&item.status)? {
                TaskStatus::PENDING => pending.push(item),
                TaskStatus::DONE => done.push(item),
            }
        }

        Ok(Self { pending, done })
    }
}

#[cfg(test)]
mod tests {
    use super::{AllTodoItems, ToDoItem};
    use glue::errors::NanoServiceErrorStatus;

    #[test]
    fn groups_items_by_status_without_losing_names() {
        let all = AllTodoItems::from_vec(vec![
            ToDoItem {
                name: "write-tests".into(),
                status: "PENDING".into(),
            },
            ToDoItem {
                name: "wire-ingress".into(),
                status: "DONE".into(),
            },
        ])
        .unwrap();

        assert_eq!(all.pending.len(), 1);
        assert_eq!(all.pending[0].name, "write-tests");
        assert_eq!(all.done.len(), 1);
        assert_eq!(all.done[0].name, "wire-ingress");
    }

    #[test]
    fn rejects_items_with_unknown_status() {
        let error = AllTodoItems::from_vec(vec![ToDoItem {
            name: "invalid".into(),
            status: "BLOCKED".into(),
        }])
        .unwrap_err();

        assert_eq!(error.status, NanoServiceErrorStatus::BadRequest);
    }
}
