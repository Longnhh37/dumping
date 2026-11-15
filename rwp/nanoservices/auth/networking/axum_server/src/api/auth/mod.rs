pub mod login;
pub mod logout;

use auth_dal::users::transactions::get::GetByEmail;
use axum::{Router, routing::post};

use super::AppState;

pub fn auth_router<T: GetByEmail + Clone + Send + Sync + 'static>() -> Router<AppState<T>> {
    Router::new().route("/auth/login", post(login::login::<T>))
}
