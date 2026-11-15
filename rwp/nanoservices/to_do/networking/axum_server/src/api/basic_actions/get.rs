use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};

use glue::{errors::NanoServiceError, token::HeaderToken};
use to_do_core::api::basic_actions::get::{
    get_all as get_all_core, get_by_name as get_by_name_core,
};
use to_do_dal::to_do_items::transactions::get::{GetAll, GetByName};

pub async fn get_all<T: GetAll>(
    State(store): State<T>,
    _token: HeaderToken,
) -> Result<impl IntoResponse, NanoServiceError> {
    let items = get_all_core(&store).await?;
    Ok((StatusCode::OK, Json(items)).into_response())
}

pub async fn get_by_name<T: GetByName>(
    State(store): State<T>,
    _token: HeaderToken,
    Path(name): Path<String>,
) -> Result<impl IntoResponse, NanoServiceError> {
    let item = get_by_name_core(&store, &name).await?;
    Ok((StatusCode::OK, Json(item)).into_response())
}
