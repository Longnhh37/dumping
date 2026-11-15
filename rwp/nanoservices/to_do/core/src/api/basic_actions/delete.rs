use glue::errors::NanoServiceError;
use to_do_dal::to_do_items::transactions::delete::DeleteOne;

pub async fn delete<T: DeleteOne>(store: &T, name: &str) -> Result<(), NanoServiceError> {
    store.delete_one(name.to_string()).await?;
    Ok(())
}
