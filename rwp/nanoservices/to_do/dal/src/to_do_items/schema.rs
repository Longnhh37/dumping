use std::collections::HashMap;
use std::fmt;

use serde::{Deserialize, Serialize};

use super::enums::TaskStatus;
use glue::errors::NanoServiceError;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NewToDoItem {
    pub title: String,
    pub status: TaskStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "sqlx-postgres", derive(sqlx::FromRow))]
pub struct ToDoItem {
    pub id: i32,
    pub title: String,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AllTodoItems {
    pub pending: Vec<ToDoItem>,
    pub done: Vec<ToDoItem>,
}
