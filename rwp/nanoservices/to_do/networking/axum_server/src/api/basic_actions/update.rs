use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use glue::{errors::NanoServiceError, token::HeaderToken};
use to_do_core::api::basic_actions::{get::get_all as get_all_core, update::update as update_core};
use to_do_dal::to_do_items::{
    schema::ToDoItem, transactions::get::GetAll, transactions::update::UpdateOne,
};

use crate::{api::AppState, auth_user::ResolveAuthUser};

pub async fn update<T: UpdateOne + GetAll, R: ResolveAuthUser>(
    State(state): State<AppState<T, R>>,
    token: HeaderToken,
    Json(body): Json<ToDoItem>,
) -> Result<impl IntoResponse, NanoServiceError> {
    let owner_key = state.auth_user.resolve_owner_key(&token.unique_id).await?;
    update_core(&state.store, body, &owner_key).await?;
    let all_items = get_all_core(&state.store, &owner_key).await?;
    Ok((StatusCode::OK, Json(all_items)))
}
