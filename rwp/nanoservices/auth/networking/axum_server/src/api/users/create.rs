use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::api::AppState;
use auth_core::api::users::create::{CreateUser, create as create_core};
use auth_dal::users::transactions::create::SaveOne;
use glue::errors::NanoServiceError;

pub async fn create<T: SaveOne + Send + Sync + 'static, X: Send + Sync + 'static>(
    State(state): State<AppState<T, X>>,
    Json(body): Json<CreateUser>,
) -> Result<impl IntoResponse, NanoServiceError> {
    create_core(&state.store, body).await?;
    Ok(StatusCode::CREATED)
}
