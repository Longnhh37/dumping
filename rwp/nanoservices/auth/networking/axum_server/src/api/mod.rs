pub mod auth;
pub mod users;

use auth_dal::users::transactions::AuthStore;
use axum::Router;

#[derive(Clone)]
pub struct AppState<T> {
    pub store: T,
}

impl<T> AppState<T> {
    pub fn new(store: T) -> Self {
        Self { store }
    }
}

pub fn router<T>(state: AppState<T>) -> Router
where
    T: AuthStore + Clone + Send + Sync + 'static,
{
    let api = Router::new()
        .merge(users::users_router::<T>())
        .merge(auth::auth_router::<T>());

    Router::new().nest("/api/v1", api).with_state(state)
}
