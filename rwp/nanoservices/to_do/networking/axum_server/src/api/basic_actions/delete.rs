use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use glue::{errors::NanoServiceError, token::HeaderToken};
use to_do_core::api::basic_actions::{delete::delete as delete_core, get::get_all as get_all_core};
use to_do_dal::to_do_items::transactions::{delete::DeleteOne, get::GetAll};

pub async fn delete_by_name<T: DeleteOne + GetAll>(
    State(store): State<T>,
    _token: HeaderToken,
    Path(name): Path<String>,
) -> Result<impl IntoResponse, NanoServiceError> {
    delete_core(&store, &name).await?;
    Ok((StatusCode::OK, Json(get_all_core(&store).await?)).into_response())
}
