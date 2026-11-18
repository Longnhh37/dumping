use glue::errors::NanoServiceError;
use to_do_dal::to_do_items::transactions::delete::DeleteOne;

pub async fn delete<T: DeleteOne>(
    store: &T,
    id: i32,
    user_id: &str,
) -> Result<(), NanoServiceError> {
    store.delete_one(id, user_id).await?;
    Ok(())
}
