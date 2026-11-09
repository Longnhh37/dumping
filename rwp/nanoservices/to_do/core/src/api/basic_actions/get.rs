use crate::structs::{AllTodoItems, TodoItem};
use glue::errors::{NanoServiceError, NanoServiceErrorStatus};
use to_do_dal::json_file::get_all as get_all_handle;

pub async fn get_all() -> Result<AllTodoItems, NanoServiceError> {
    Ok(AllTodoItems::from(get_all_handle::<TodoItem>()?))
}

pub async fn get_by_name(id: u64) -> Result<TodoItem, NanoServiceError> {
    get_all_handle()?.remove(&id.to_string()).ok_or_else(|| {
        NanoServiceError::new(
            format!("Item with id '{}' not found", id),
            NanoServiceErrorStatus::NotFound,
        )
    })
}
