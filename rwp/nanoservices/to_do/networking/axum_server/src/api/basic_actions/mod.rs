pub mod create;
pub mod delete;
pub mod get;
pub mod update;

use auth_kernel::user_session::{
    descriptors::RedisSessionDescriptor, transactions::get::GetUserSession,
};
use axum::{
    Router,
    routing::{delete, get, patch, post},
};
use to_do_dal::to_do_items::transactions::{
    create::SaveOne,
    delete::DeleteOne,
    get::{GetAll, GetById},
    update::UpdateOne,
};

use crate::api::AppState;

fn router<T, X>(store: T, user_session: X) -> Router
where
    T: GetAll + GetById + DeleteOne + SaveOne + UpdateOne + Clone + Send + Sync + 'static,
    X: GetUserSession + Clone + Send + Sync + 'static,
{
    let api_v1 = Router::new()
        .route("/get", get(get::get_all::<T, X>))
        .route("/get/{id}", get(get::get_by_id::<T, X>))
        .route("/delete/{id}", delete(delete::delete_by_id::<T, X>))
        .route("/create", post(create::create::<T, X>))
        .route("/update", patch(update::update::<T, X>));

    Router::new()
        .nest("/api/v1", api_v1)
        .with_state(AppState::new(store, user_session))
}

pub fn basic_actions_router_with_session<T, X>(store: T, user_session: X) -> Router
where
    T: GetAll + GetById + DeleteOne + SaveOne + UpdateOne + Clone + Send + Sync + 'static,
    X: GetUserSession + Clone + Send + Sync + 'static,
{
    router(store, user_session)
}

#[cfg(all(feature = "auth-http", not(feature = "auth-core-postgres")))]
pub fn basic_actions_router_http<T>(store: T) -> Router
where
    T: GetAll + GetById + DeleteOne + SaveOne + UpdateOne + Clone + Send + Sync + 'static,
{
    router(store, RedisSessionDescriptor)
}

#[cfg(feature = "auth-core-postgres")]
pub fn basic_actions_router_core_postgres<T, A>(store: T, auth_store: A) -> Router
where
    T: GetAll + GetById + DeleteOne + SaveOne + UpdateOne + Clone + Send + Sync + 'static,
    A: auth_dal::users::transactions::get::GetByUniqueId + Clone + Send + Sync + 'static,
{
    router(store, RedisSessionDescriptor::new(auth_store))
}
