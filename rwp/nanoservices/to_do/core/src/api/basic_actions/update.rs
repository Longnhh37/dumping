use crate::structs::TodoItem;
use glue::errors::{NanoServiceError, NanoServiceErrorStatus};
use to_do_dal::json_file::{get_all as get_all_handle, save_all};

pub async fn update(item: TodoItem) -> Result<(), NanoServiceError> {
    let mut all_items = get_all_handle::<TodoItem>()?;
    if !all_items.contains_key(&item.title) {
        return Err(NanoServiceError::new(
            format!("Item with name {} not found", item.title),
            NanoServiceErrorStatus::NotFound,
        ));
    }
    all_items.insert(item.title.clone(), item);
    save_all(&all_items)?;
    Ok(())
}
