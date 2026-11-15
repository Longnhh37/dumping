pub mod create;

use axum::{Router, routing::post};

use auth_dal::users::transactions::create::SaveOne;

pub fn users_router<T: SaveOne + Clone + Send + Sync + 'static>() -> Router<T> {
    Router::new().route("/users/create", post(create::create::<T>))
}
