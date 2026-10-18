use axum::{Json, extract::Path};
use glue::errors::NanoServiceError;
use to_do_core::api::basic_actions::get::{
    get_all as get_all_core, get_by_name as get_by_name_core,
};
use to_do_core::structs::{AllTodoItems, TodoItem};

pub async fn get_all() -> Result<Json<AllTodoItems>, NanoServiceError> {
    Ok(Json(get_all_core().await?))
}

pub async fn get_by_name(Path(name): Path<String>) -> Result<Json<TodoItem>, NanoServiceError> {
    Ok(Json(get_by_name_core(&name).await?))
}
