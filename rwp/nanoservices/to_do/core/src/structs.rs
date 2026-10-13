use crate::enums::TaskStatus;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AllTodoItems {
    pub pending: Vec<TodoItem>,
    pub done: Vec<TodoItem>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TodoItem {
    pub title: String,
    pub status: TaskStatus,
}

impl From<HashMap<String, TodoItem>> for AllTodoItems {
    fn from(all_items: HashMap<String, TodoItem>) -> Self {
        let mut pending = Vec::new();
        let mut done = Vec::new();
        for (_, item) in all_items {
            match item.status {
                TaskStatus::Pending => pending.push(item),
                TaskStatus::Done => done.push(item),
            }
        }

        AllTodoItems { pending, done }
    }
}

impl fmt::Display for TodoItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.status {
            TaskStatus::Pending => write!(f, "Pending: {}", self.title),
            TaskStatus::Done => write!(f, "Done: {}", self.title),
        }
    }
}
