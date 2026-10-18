use axum::Json;
use glue::errors::NanoServiceError;
use to_do_core::api::basic_actions::{create::create as create_core, get::get_all as get_all_core};
use to_do_core::structs::TodoItem;
use to_do_core::structs::AllTodoItems;

pub async fn create(Json(body): Json<TodoItem>) -> Result<Json<AllTodoItems>, NanoServiceError> {
    create_core(body).await?;
    Ok(Json(get_all_core().await?))
}
