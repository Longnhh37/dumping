use glue::errors::NanoServiceError;
use to_do_dal::to_do_items::schema::ToDoItem;
use to_do_dal::to_do_items::transactions::update::UpdateOne;

pub async fn update<T: UpdateOne>(store: &T, item: ToDoItem) -> Result<(), NanoServiceError> {
    store.update_one(item).await?;
    Ok(())
}
