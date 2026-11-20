pub mod login;
pub mod logout;

use auth_dal::users::transactions::get::GetByEmail;
use auth_kernel::user_session::transactions::login::LoginUserSession;
use axum::{Router, routing::post};

use super::AppState;

pub fn auth_router<T, X>() -> Router<AppState<T, X>>
where
    T: GetByEmail + Clone + Send + Sync + 'static,
    X: LoginUserSession + Clone + Send + Sync + 'static,
{
    Router::new().route("/auth/login", post(login::login::<T, X>))
}
