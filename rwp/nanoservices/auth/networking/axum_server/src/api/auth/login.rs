use axum::{
    Json,
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    response::IntoResponse,
};

use crate::api::AppState;
use crate::extract_auth::extract_credentials;
use auth_core::api::auth::login::login as core_login;
use auth_dal::users::transactions::get::GetByEmail;
use glue::errors::NanoServiceError;

pub async fn login<T: GetByEmail>(
    State(state): State<AppState<T>>,
    req: Request<Body>,
) -> Result<impl IntoResponse, NanoServiceError> {
    let creds = extract_credentials(&req)?;
    let token = core_login(&state.store, creds.email, creds.password).await?;
    Ok((StatusCode::OK, Json(token)))
}
