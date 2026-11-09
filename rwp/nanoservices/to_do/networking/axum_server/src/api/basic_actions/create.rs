use axum::Json;
use axum::http::StatusCode;
use glue::errors::NanoServiceError;
use to_do_core::api::basic_actions::{create::create as create_core, get::get_all as get_all_core};
use to_do_core::structs::AllTodoItems;
use to_do_core::structs::TodoItem;

pub async fn create(
    Json(body): Json<TodoItem>,
) -> Result<(StatusCode, Json<AllTodoItems>), NanoServiceError> {
    create_core(body).await?;
    Ok((StatusCode::CREATED, Json(get_all_core().await?)))
}
