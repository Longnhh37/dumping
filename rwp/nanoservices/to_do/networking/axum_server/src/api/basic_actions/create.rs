use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use glue::{errors::NanoServiceError, token::HeaderToken};
use to_do_core::api::basic_actions::{create::create as create_core, get::get_all as get_all_core};
use to_do_dal::to_do_items::schema::NewToDoItem;
use to_do_dal::to_do_items::transactions::{create::SaveOne, get::GetAll};

use crate::{api::AppState, auth_user::ResolveAuthUser};

pub async fn create<T: SaveOne + GetAll, R: ResolveAuthUser>(
    State(state): State<AppState<T, R>>,
    token: HeaderToken,
    Json(body): Json<NewToDoItem>,
) -> Result<impl IntoResponse, NanoServiceError> {
    let owner_key = state.auth_user.resolve_owner_key(&token.unique_id).await?;
    create_core(&state.store, body, &owner_key).await?;
    Ok((
        StatusCode::CREATED,
        Json(get_all_core(&state.store, &owner_key).await?),
    ))
}
