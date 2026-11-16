use glue::errors::NanoServiceError;
use to_do_dal::to_do_items::schema::ToDoItem;
use to_do_dal::to_do_items::transactions::create::SaveOne;

pub async fn create<T: SaveOne>(
    store: &T,
    item: ToDoItem,
) -> Result<ToDoItem, NanoServiceError> {
    let created_item = store.save_one(item).await?;
    Ok(created_item)
}
