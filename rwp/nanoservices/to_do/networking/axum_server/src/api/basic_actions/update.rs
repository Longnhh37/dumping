use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use auth_kernel::user_session::transactions::get::GetUserSession;
use glue::{errors::NanoServiceError, token::HeaderToken};
use to_do_core::api::basic_actions::{get::get_all as get_all_core, update::update as update_core};
use to_do_dal::to_do_items::{
    schema::ToDoItem, transactions::get::GetAll, transactions::update::UpdateOne,
};

use crate::api::AppState;

pub async fn update<T: UpdateOne + GetAll, X: GetUserSession>(
    State(state): State<AppState<T, X>>,
    token: HeaderToken,
    Json(body): Json<ToDoItem>,
) -> Result<impl IntoResponse, NanoServiceError> {
    let session = state
        .user_session
        .get_user_session(&token.unique_id)
        .await?;
    let owner_key = session.user_id.to_string();
    update_core(&state.store, body, &owner_key).await?;
    let all_items = get_all_core(&state.store, &owner_key).await?;
    Ok((StatusCode::OK, Json(all_items)))
}
