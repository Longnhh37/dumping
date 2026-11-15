pub mod create;

use axum::{Router, routing::post};

use auth_dal::users::transactions::create::SaveOne;

use super::AppState;

pub fn users_router<T: SaveOne + Clone + Send + Sync + 'static>() -> Router<AppState<T>> {
    Router::new().route("/users/create", post(create::create::<T>))
}
