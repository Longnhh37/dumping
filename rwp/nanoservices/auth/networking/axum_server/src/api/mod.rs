pub mod auth;
pub mod users;

use axum::Router;

pub fn router() -> Router {
    let api = Router::new()
        .merge(users::users_router())
        .merge(auth::auth_router());

    Router::new().nest("/api/v1", api)
}
