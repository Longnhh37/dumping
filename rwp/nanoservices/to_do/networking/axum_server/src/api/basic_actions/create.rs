use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use glue::{errors::NanoServiceError, token::HeaderToken};
use to_do_core::api::basic_actions::{create::create as create_core, get::get_all as get_all_core};
use to_do_dal::to_do_items::schema::NewToDoItem;
use to_do_dal::to_do_items::transactions::{create::SaveOne, get::GetAll};

use crate::api::AppState;

pub async fn create<T: SaveOne + GetAll>(
    State(state): State<AppState<T>>,
    token: HeaderToken,
    Json(body): Json<NewToDoItem>,
) -> Result<impl IntoResponse, NanoServiceError> {
    create_core(&state.store, body, &token.unique_id).await?;
    Ok((
        StatusCode::CREATED,
        Json(get_all_core(&state.store, &token.unique_id).await?),
    ))
}
