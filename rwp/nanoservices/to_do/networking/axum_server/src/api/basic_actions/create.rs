use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use glue::{errors::NanoServiceError, token::HeaderToken};
use to_do_core::api::basic_actions::{create::create as create_core, get::get_all as get_all_core};
use to_do_dal::to_do_items::schema::NewToDoItem;
use to_do_dal::to_do_items::transactions::{create::SaveOne, get::GetAll};

pub async fn create<T: SaveOne + GetAll>(
    State(store): State<T>,
    _token: HeaderToken,
    Json(body): Json<NewToDoItem>,
) -> Result<impl IntoResponse, NanoServiceError> {
    create_core(&store, body).await?;
    Ok((StatusCode::CREATED, Json(get_all_core(&store).await?)))
}
