use glue::errors::NanoServiceError;
use to_do_dal::to_do_items::schema::{NewToDoItem, ToDoItem};
use to_do_dal::to_do_items::transactions::create::SaveOne;

// #[cfg(feature = "json-file-storage")]
// use to_do_dal::json_file::{allocate_id, get_all, save_one};
//
// pub async fn create<T: SaveOne>(item: NewToDoItem) -> Result<ToDoItem, NanoServiceError> {
//     let all_items = get_all::<TodoItem>()?;
//     let max_id = all_items.values().map(|item| item.id).max().unwrap_or(0);
//     let item = TodoItem {
//         id: allocate_id(max_id)?,
//         title: item.title,
//         status: item.status,
//     };
//     save_one(&item.id.to_string(), &item)?;
//
//     Ok(item)
// }

pub async fn create<T: SaveOne>(item: NewToDoItem) -> Result<ToDoItem, NanoServiceError> {
    let created_item = T::save_one(item).await?;
    Ok(created_item)
}
