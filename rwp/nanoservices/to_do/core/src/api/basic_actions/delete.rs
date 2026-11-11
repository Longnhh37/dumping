// TODO: CLEAN THIS CIO
// use crate::structs::TodoItem;
// use to_do_dal::json_file::delete_one;
use glue::errors::NanoServiceError;
use to_do_dal::to_do_items::transactions::delete::DeleteOne;

// pub async fn delete(id: u64) -> Result<TodoItem, NanoServiceError> {
//     delete_one::<TodoItem>(&id.to_string())
// }

pub async fn delete<T: DeleteOne>(id: &str) -> Result<(), NanoServiceError> {
    T::delete_one(id.to_string()).await?;
    Ok(())
}
