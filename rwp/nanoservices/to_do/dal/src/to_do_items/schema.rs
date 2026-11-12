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
