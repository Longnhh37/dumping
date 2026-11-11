use axum::{Json, extract::Path, http::StatusCode, response::IntoResponse};

use glue::errors::NanoServiceError;
use to_do_core::api::basic_actions::{delete::delete as delete_core, get::get_all as get_all_core};
use to_do_dal::to_do_items::transactions::{delete::DeleteOne, get::GetAll};

pub async fn delete_by_name<T: DeleteOne + GetAll>(
    Path(name): Path<String>,
) -> Result<impl IntoResponse, NanoServiceError> {
    delete_core::<T>(&name).await?;
    Ok((StatusCode::OK, Json(get_all_core::<T>().await?)).into_response())
}
