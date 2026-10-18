use axum::Json;
use axum::response::IntoResponse;
use glue::{errors::NanoServiceError, token::HeaderToken};
use to_do_core::api::basic_actions::{get::get_all as get_all_core, update::update as update_core};
use to_do_core::structs::TodoItem;

pub async fn update(
    token: HeaderToken,
    Json(body): Json<TodoItem>,
) -> Result<impl IntoResponse, NanoServiceError> {
    println!("Token: {}", token.message);
    update_core(body).await?;
    Ok(Json(get_all_core().await?))
}
