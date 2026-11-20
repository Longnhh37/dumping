use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use auth_kernel::user_session::transactions::get::GetUserSession;
use glue::{errors::NanoServiceError, token::HeaderToken};
use to_do_core::api::basic_actions::{create::create as create_core, get::get_all as get_all_core};
use to_do_dal::to_do_items::schema::NewToDoItem;
use to_do_dal::to_do_items::transactions::{create::SaveOne, get::GetAll};

use crate::api::AppState;

pub async fn create<T: SaveOne + GetAll, X: GetUserSession>(
    State(state): State<AppState<T, X>>,
    token: HeaderToken,
    Json(body): Json<NewToDoItem>,
) -> Result<impl IntoResponse, NanoServiceError> {
    let session = state
        .user_session
        .get_user_session(&token.unique_id)
        .await?;
    let owner_key = session.user_id.to_string();
    create_core(&state.store, body, &owner_key).await?;
    Ok((
        StatusCode::CREATED,
        Json(get_all_core(&state.store, &owner_key).await?),
    ))
}
