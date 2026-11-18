use glue::errors::NanoServiceError;
use to_do_dal::to_do_items::schema::ToDoItem;
use to_do_dal::to_do_items::transactions::update::UpdateOne;

pub async fn update<T: UpdateOne>(
    store: &T,
    item: ToDoItem,
    user_id: &str,
) -> Result<(), NanoServiceError> {
    store.update_one(item, user_id).await?;
    Ok(())
}
