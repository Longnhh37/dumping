use crate::structs::TodoItem;
use glue::errors::NanoServiceError;
use to_do_dal::json_file::delete_one;

pub async fn delete(id: &str) -> Result<TodoItem, NanoServiceError> {
    delete_one::<TodoItem>(id)
}
