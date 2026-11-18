use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use glue::{errors::NanoServiceError, token::HeaderToken};
use to_do_core::api::basic_actions::{get::get_all as get_all_core, update::update as update_core};
use to_do_dal::to_do_items::{
    schema::ToDoItem, transactions::get::GetAll, transactions::update::UpdateOne,
};

use crate::api::AppState;

pub async fn update<T: UpdateOne + GetAll>(
    State(state): State<AppState<T>>,
    token: HeaderToken,
    Json(body): Json<ToDoItem>,
) -> Result<impl IntoResponse, NanoServiceError> {
    update_core(&state.store, body, &token.unique_id).await?;
    let all_items = get_all_core(&state.store, &token.unique_id).await?;
    Ok((StatusCode::OK, Json(all_items)))
}
