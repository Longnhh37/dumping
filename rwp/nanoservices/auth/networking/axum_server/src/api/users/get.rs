use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use auth_core::api::users::get::get_by_unique_id as get_by_unique_id_core;
use auth_dal::users::transactions::get::GetByUniqueId;
use glue::{errors::NanoServiceError, token::HeaderToken};

use crate::api::AppState;

pub async fn get_me<T: GetByUniqueId>(
    State(state): State<AppState<T>>,
    token: HeaderToken,
) -> Result<impl IntoResponse, NanoServiceError> {
    let user = get_by_unique_id_core::<T>(&state.store, &token.unique_id).await?;
    Ok((StatusCode::OK, Json(user)))
}
