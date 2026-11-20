pub mod create;
pub mod get;

use axum::{
    Router,
    routing::{get, post},
};

use auth_dal::users::transactions::AuthStore;

use super::AppState;

pub fn users_router<
    T: AuthStore + Clone + Send + Sync + 'static,
    X: Clone + Send + Sync + 'static,
>() -> Router<AppState<T, X>> {
    let users_router = Router::new()
        .route("/create", post(create::create::<T, X>))
        .route("/me", get(get::get_me::<T, X>));

    Router::new().nest("/users", users_router)
}
