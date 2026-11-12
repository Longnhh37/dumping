use glue::errors::NanoServiceError;
use to_do_dal::to_do_items::{
    schema::{AllTodoItems, ToDoItem},
    transactions::get::{GetAll, GetByName},
};

pub async fn get_all<T: GetAll>() -> Result<AllTodoItems, NanoServiceError> {
    let all_items = T::get_all().await?;
    AllTodoItems::from_vec(all_items)
}

pub async fn get_by_name<T: GetByName>(name: &str) -> Result<ToDoItem, NanoServiceError> {
    T::get_by_name(name.to_owned()).await
}
