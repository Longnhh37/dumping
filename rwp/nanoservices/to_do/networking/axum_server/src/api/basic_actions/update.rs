use axum::{Json, http::StatusCode, response::IntoResponse};

use glue::{errors::NanoServiceError, token::HeaderToken};
use to_do_core::api::basic_actions::{get::get_all as get_all_core, update::update as update_core};
use to_do_dal::to_do_items::{
    schema::ToDoItem, transactions::get::GetAll, transactions::update::UpdateOne,
};

pub async fn update<T: UpdateOne + GetAll>(
    _token: HeaderToken,
    Json(body): Json<ToDoItem>,
) -> Result<impl IntoResponse, NanoServiceError> {
    update_core::<T>(body).await?;
    let all_items = get_all_core::<T>().await?;
    Ok((StatusCode::OK, Json(all_items)))
}
