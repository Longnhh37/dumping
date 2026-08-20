use crate::structs::TodoItem;
use glue::errors::NanoServiceError;

#[cfg(feature = "json-file-storage")]
use to_do_dal::json_file::save_one;

pub async fn create(item: TodoItem) -> Result<TodoItem, NanoServiceError> {
    save_one(&item.title.to_string(), &item)?;

    Ok(item)
}
