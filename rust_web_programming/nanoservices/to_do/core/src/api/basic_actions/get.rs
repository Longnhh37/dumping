use crate::structs::{AllTodoItems, TodoItem};
use glue::errors::{NanoServiceError, NanoServiceErrorStatus};
use to_do_dal::json_file::get_all as get_all_handle;

pub async fn get_all() -> Result<AllTodoItems, NanoServiceError> {
    Ok(AllTodoItems::from(get_all_handle::<TodoItem>()?))
}

pub async fn get_by_name(name: &str) -> Result<TodoItem, NanoServiceError> {
    get_all_handle()?.remove(name).ok_or(NanoServiceError::new(
        format!("Item with name '{}' not found", name),
        NanoServiceErrorStatus::NotFound,
    ))
}
