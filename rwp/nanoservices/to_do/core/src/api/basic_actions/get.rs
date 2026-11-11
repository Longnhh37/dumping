use glue::errors::NanoServiceError;
use to_do_dal::to_do_items::{schema::AllTodoItems, transactions::get::GetAll};

pub async fn get_all<T: GetAll>() -> Result<AllTodoItems, NanoServiceError> {
    let all_items = T::get_all().await?;
    AllTodoItems::from_vec(all_items)
}
