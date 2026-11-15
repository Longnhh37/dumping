use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use auth_core::api::users::create::{CreateUser, create as create_core};
use auth_dal::users::transactions::create::SaveOne;
use glue::errors::NanoServiceError;

pub async fn create<T: SaveOne>(
    State(store): State<T>,
    Json(body): Json<CreateUser>,
) -> Result<impl IntoResponse, NanoServiceError> {
    create_core(&store, body).await?;
    Ok(StatusCode::CREATED)
}
