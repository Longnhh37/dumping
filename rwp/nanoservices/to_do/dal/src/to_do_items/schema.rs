use serde::{Deserialize, Serialize};

use super::enums::TaskStatus;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "sqlx-postgres", derive(sqlx::FromRow))]
pub struct ToDoItem {
    pub id: i32,
    pub title: String,
    #[cfg_attr(feature = "sqlx-postgres", sqlx(try_from = "String"))]
    pub status: TaskStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct NewToDoItem {
    pub title: String,
    pub status: TaskStatus,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AllTodoItems {
    pub pending: Vec<ToDoItem>,
    pub done: Vec<ToDoItem>,
}

impl AllTodoItems {
    pub fn from_vec(all_items: Vec<ToDoItem>) -> Self {
        let mut pending = Vec::new();
        let mut done = Vec::new();

        for item in all_items {
            match item.status {
                TaskStatus::Pending => pending.push(item),
                TaskStatus::Done => done.push(item),
            }
        }

        Self { pending, done }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[cfg_attr(feature = "sqlx-postgres", derive(sqlx::FromRow))]
pub struct UserConnection {
    pub user_id: String,
    pub to_do_id: i32,
}

#[cfg(test)]
mod tests {
    use super::{AllTodoItems, TaskStatus, ToDoItem};

    #[test]
    fn groups_items_by_status_without_losing_ids_and_titles() {
        let all = AllTodoItems::from_vec(vec![
            ToDoItem {
                id: 1,
                title: "write-tests".into(),
                status: TaskStatus::Pending,
            },
            ToDoItem {
                id: 2,
                title: "wire-ingress".into(),
                status: TaskStatus::Done,
            },
        ]);

        assert_eq!(all.pending.len(), 1);
        assert_eq!(all.pending[0].id, 1);
        assert_eq!(all.pending[0].title, "write-tests");
        assert_eq!(all.done.len(), 1);
        assert_eq!(all.done[0].id, 2);
        assert_eq!(all.done[0].title, "wire-ingress");
    }
}
