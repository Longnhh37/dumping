pub mod create;
pub mod get;

use axum::{
    Router,
    routing::{get, post},
};

use auth_dal::users::transactions::AuthStore;

use super::AppState;

pub fn users_router<T: AuthStore + Clone + Send + Sync + 'static>() -> Router<AppState<T>> {
    let users_router = Router::new()
        .route("/create", post(create::create::<T>))
        .route("/me", get(get::get_me::<T>));

    Router::new().nest("/users", users_router)
}
