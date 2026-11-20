use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use auth_kernel::user_session::transactions::get::GetUserSession;
use glue::{errors::NanoServiceError, token::HeaderToken};
use to_do_core::api::basic_actions::get::{get_all as get_all_core, get_by_id as get_by_id_core};
use to_do_dal::to_do_items::transactions::get::{GetAll, GetById};

use crate::api::AppState;

pub async fn get_all<T: GetAll, X: GetUserSession>(
    State(state): State<AppState<T, X>>,
    token: HeaderToken,
) -> Result<impl IntoResponse, NanoServiceError> {
    let session = state
        .user_session
        .get_user_session(&token.unique_id)
        .await?;
    let owner_key = session.user_id.to_string();
    let items = get_all_core(&state.store, &owner_key).await?;
    Ok((StatusCode::OK, Json(items)).into_response())
}

pub async fn get_by_id<T: GetById, X: GetUserSession>(
    State(state): State<AppState<T, X>>,
    token: HeaderToken,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, NanoServiceError> {
    let session = state
        .user_session
        .get_user_session(&token.unique_id)
        .await?;
    let owner_key = session.user_id.to_string();
    let item = get_by_id_core(&state.store, id, &owner_key).await?;
    Ok((StatusCode::OK, Json(item)).into_response())
}
