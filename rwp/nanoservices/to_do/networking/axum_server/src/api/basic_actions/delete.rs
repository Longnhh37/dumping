use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use auth_kernel::user_session::transactions::get::GetUserSession;
use glue::{errors::NanoServiceError, token::HeaderToken};
use to_do_core::api::basic_actions::{delete::delete as delete_core, get::get_all as get_all_core};
use to_do_dal::to_do_items::transactions::{delete::DeleteOne, get::GetAll};

use crate::api::AppState;

pub async fn delete_by_id<T: DeleteOne + GetAll, X: GetUserSession>(
    State(state): State<AppState<T, X>>,
    token: HeaderToken,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, NanoServiceError> {
    let session = state
        .user_session
        .get_user_session(&token.unique_id)
        .await?;
    let owner_key = session.user_id.to_string();
    delete_core(&state.store, id, &owner_key).await?;
    Ok((
        StatusCode::OK,
        Json(get_all_core(&state.store, &owner_key).await?),
    )
        .into_response())
}
