pub mod auth;
pub mod users;

use auth_dal::users::transactions::AuthStore;
use auth_kernel::user_session::transactions::login::LoginUserSession;
use auth_kernel::user_session::transactions::logout::LogoutUserSession;
use axum::Router;

pub use auth_kernel::user_session::descriptors::RedisSessionDescriptor;

#[derive(Clone)]
pub struct AppState<T, X> {
    pub store: T,
    pub user_session: X,
}

impl<T, X> AppState<T, X> {
    pub fn new(store: T, user_session: X) -> Self {
        Self {
            store,
            user_session,
        }
    }
}

pub fn router<T, X>(state: AppState<T, X>) -> Router
where
    T: AuthStore + Clone + Send + Sync + 'static,
    X: LoginUserSession + LogoutUserSession + Clone + Send + Sync + 'static,
{
    let api = Router::new()
        .merge(users::users_router::<T, X>())
        .merge(auth::auth_router::<T, X>());

    Router::new().nest("/api/v1", api).with_state(state)
}
