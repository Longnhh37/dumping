use axum::{extract::State, http::StatusCode, response::IntoResponse};

use auth_kernel::user_session::transactions::logout::LogoutUserSession;
use glue::{
    errors::{NanoServiceError, NanoServiceErrorStatus, ResultExt},
    token::HeaderToken,
};

use crate::api::AppState;

pub async fn logout<T: Send + Sync + 'static, X: LogoutUserSession>(
    State(state): State<AppState<T, X>>,
    token: HeaderToken,
) -> Result<impl IntoResponse, NanoServiceError> {
    let cache_address = std::env::var("CACHE_API_URL").eject(NanoServiceErrorStatus::BadRequest)?;
    state
        .user_session
        .logout_user_session(&cache_address, &token.unique_id)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
