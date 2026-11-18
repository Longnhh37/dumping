use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::{api::AppState, extract_auth::Credentials};
use auth_core::api::auth::login::login as core_login;
use auth_dal::users::transactions::get::GetByEmail;
use glue::errors::NanoServiceError;

pub async fn login<T: GetByEmail>(
    State(state): State<AppState<T>>,
    creds: Credentials,
) -> Result<impl IntoResponse, NanoServiceError> {
    let token = core_login(&state.store, &creds.email, creds.password).await?;
    Ok((StatusCode::OK, Json(token)))
}
