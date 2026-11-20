pub mod login;
pub mod logout;

use auth_dal::users::transactions::get::GetByEmail;
use auth_kernel::user_session::transactions::login::LoginUserSession;
use auth_kernel::user_session::transactions::logout::LogoutUserSession;
use axum::{Router, routing::post};

use super::AppState;

pub fn auth_router<T, X>() -> Router<AppState<T, X>>
where
    T: GetByEmail + Clone + Send + Sync + 'static,
    X: LoginUserSession + LogoutUserSession + Clone + Send + Sync + 'static,
{
    let auth = Router::new()
        .route("/login", post(login::login::<T, X>))
        .route("/logout", post(logout::logout::<T, X>));

    Router::new().nest("/auth", auth)
}
