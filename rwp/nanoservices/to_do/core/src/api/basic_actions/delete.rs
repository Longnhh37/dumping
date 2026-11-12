use glue::errors::NanoServiceError;
use to_do_dal::to_do_items::transactions::delete::DeleteOne;

pub async fn delete<T: DeleteOne>(name: &str) -> Result<(), NanoServiceError> {
    T::delete_one(name.to_string()).await?;
    Ok(())
}
