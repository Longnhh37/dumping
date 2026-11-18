use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use glue::{errors::NanoServiceError, token::HeaderToken};
use to_do_core::api::basic_actions::{delete::delete as delete_core, get::get_all as get_all_core};
use to_do_dal::to_do_items::transactions::{delete::DeleteOne, get::GetAll};

use crate::{api::AppState, auth_user::ResolveAuthUser};

pub async fn delete_by_id<T: DeleteOne + GetAll, R: ResolveAuthUser>(
    State(state): State<AppState<T, R>>,
    token: HeaderToken,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, NanoServiceError> {
    let owner_key = state.auth_user.resolve_owner_key(&token.unique_id).await?;
    delete_core(&state.store, id, &owner_key).await?;
    Ok((
        StatusCode::OK,
        Json(get_all_core(&state.store, &owner_key).await?),
    )
        .into_response())
}
