use crate::structs::TodoItem;
use glue::errors::NanoServiceError;
use to_do_dal::json_file::delete_one;

pub async fn delete(id: u64) -> Result<TodoItem, NanoServiceError> {
    delete_one::<TodoItem>(&id.to_string())
}
