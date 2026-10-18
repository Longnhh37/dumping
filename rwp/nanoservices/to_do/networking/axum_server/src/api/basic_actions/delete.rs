use axum::{Json, extract::Path};
use glue::errors::NanoServiceError;
use to_do_core::api::basic_actions::{delete::delete as delete_core, get::get_all as get_all_core};
use to_do_core::structs::AllTodoItems;

pub async fn delete_by_name(Path(name): Path<String>) -> Result<Json<AllTodoItems>, NanoServiceError> {
    delete_core(&name).await?;
    Ok(Json(get_all_core().await?))
}
