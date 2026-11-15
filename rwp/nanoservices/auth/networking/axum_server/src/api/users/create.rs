use axum::{Json, http::StatusCode, response::IntoResponse};

use auth_core::api::users::create::{CreateUser, create as create_core};
use auth_dal::users::schema::NewUser;
use auth_dal::users::transactions::create::SaveOne;
use glue::errors::NanoServiceError;

pub async fn create<T: SaveOne>(
    Json(body): Json<CreateUser>,
) -> Result<impl IntoResponse, NanoServiceError> {
    create_core::<T>(body).await?;
    Ok(StatusCode::CREATED)
}
