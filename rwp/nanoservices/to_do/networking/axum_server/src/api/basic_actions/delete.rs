use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use glue::{errors::NanoServiceError, token::HeaderToken};
use to_do_core::api::basic_actions::{delete::delete as delete_core, get::get_all as get_all_core};
use to_do_dal::to_do_items::transactions::{delete::DeleteOne, get::GetAll};

use crate::api::AppState;

pub async fn delete_by_id<T: DeleteOne + GetAll>(
    State(state): State<AppState<T>>,
    token: HeaderToken,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, NanoServiceError> {
    delete_core(&state.store, id, &token.unique_id).await?;
    Ok((
        StatusCode::OK,
        Json(get_all_core(&state.store, &token.unique_id).await?),
    )
        .into_response())
}
