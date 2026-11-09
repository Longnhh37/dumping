use crate::structs::{NewTodoItem, TodoItem};
use glue::errors::NanoServiceError;

#[cfg(feature = "json-file-storage")]
use to_do_dal::json_file::{allocate_id, get_all, save_one};

pub async fn create(item: NewTodoItem) -> Result<TodoItem, NanoServiceError> {
    let all_items = get_all::<TodoItem>()?;
    let max_id = all_items.values().map(|item| item.id).max().unwrap_or(0);
    let item = TodoItem {
        id: allocate_id(max_id)?,
        title: item.title,
        status: item.status,
    };
    save_one(&item.id.to_string(), &item)?;

    Ok(item)
}
