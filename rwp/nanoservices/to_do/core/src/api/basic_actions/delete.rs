use glue::errors::NanoServiceError;
use to_do_dal::to_do_items::transactions::delete::DeleteOne;

pub async fn delete<T: DeleteOne>(id: &str) -> Result<(), NanoServiceError> {
    T::delete_one(id.to_string()).await?;
    Ok(())
}
