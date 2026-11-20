use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};

use crate::{api::AppState, extract_auth::Credentials};
use auth_core::api::auth::login::login as core_login;
use auth_dal::users::transactions::get::GetByEmail;
use auth_kernel::user_session::transactions::login::LoginUserSession;
use glue::errors::{NanoServiceError, NanoServiceErrorStatus, ResultExt};

const SESSION_TIMEOUT_MINS: usize = 20;

pub async fn login<T: GetByEmail, X: LoginUserSession>(
    State(state): State<AppState<T, X>>,
    creds: Credentials,
) -> Result<impl IntoResponse, NanoServiceError> {
    let token = core_login(&state.store, &creds.email, creds.password).await?;
    let user = state.store.get_by_email(&creds.email).await?;
    let cache_address = std::env::var("CACHE_API_URL").eject(NanoServiceErrorStatus::BadRequest)?;
    state
        .user_session
        .login_user_session(
            &cache_address,
            &user.unique_id,
            SESSION_TIMEOUT_MINS,
            user.id,
        )
        .await?;
    Ok((StatusCode::OK, Json(token)))
}
