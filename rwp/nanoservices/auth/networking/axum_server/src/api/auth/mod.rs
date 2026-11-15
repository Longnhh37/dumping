pub mod login;
pub mod logout;

use axum::Router;

pub fn auth_router<T: Clone + Send + Sync + 'static>() -> Router<T> {
    Router::new()
}
