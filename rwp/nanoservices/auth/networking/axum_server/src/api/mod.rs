pub mod auth;
pub mod users;

use auth_dal::users::transactions::create::SaveOne;
use axum::Router;

pub fn router<T: SaveOne + Clone + Send + Sync + 'static>(store: T) -> Router {
    let api = Router::new()
        .merge(users::users_router::<T>())
        .merge(auth::auth_router::<T>());

    Router::new().nest("/api/v1", api).with_state(store)
}
