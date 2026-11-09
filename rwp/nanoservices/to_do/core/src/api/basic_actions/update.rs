use crate::structs::{TodoItem, UpdateTodoItem};
use glue::errors::{NanoServiceError, NanoServiceErrorStatus};
use to_do_dal::json_file::{get_all as get_all_handle, save_all};

pub async fn update(update: UpdateTodoItem) -> Result<(), NanoServiceError> {
    let mut all_items = get_all_handle::<TodoItem>()?;
    let key = update.id.to_string();
    let item = all_items.get_mut(&key).ok_or_else(|| NanoServiceError::new(
        format!("Item with id {} not found", update.id),
        NanoServiceErrorStatus::NotFound,
    ))?;
    item.status = update.status;
    save_all(&all_items)?;
    Ok(())
}
