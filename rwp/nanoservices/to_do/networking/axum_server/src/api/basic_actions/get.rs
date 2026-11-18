use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use glue::{errors::NanoServiceError, token::HeaderToken};
use to_do_core::api::basic_actions::get::{get_all as get_all_core, get_by_id as get_by_id_core};
use to_do_dal::to_do_items::transactions::get::{GetAll, GetById};

use crate::api::AppState;

pub async fn get_all<T: GetAll>(
    State(state): State<AppState<T>>,
    token: HeaderToken,
) -> Result<impl IntoResponse, NanoServiceError> {
    let items = get_all_core(&state.store, &token.unique_id).await?;
    Ok((StatusCode::OK, Json(items)).into_response())
}

pub async fn get_by_id<T: GetById>(
    State(state): State<AppState<T>>,
    token: HeaderToken,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, NanoServiceError> {
    let item = get_by_id_core(&state.store, id, &token.unique_id).await?;
    Ok((StatusCode::OK, Json(item)).into_response())
}
