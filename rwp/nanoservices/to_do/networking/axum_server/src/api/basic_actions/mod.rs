pub mod create;
pub mod delete;
pub mod get;
pub mod update;

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

use crate::{api::AppState, auth_user::ResolveAuthUser};

#[cfg(feature = "auth-core-postgres")]
use crate::auth_user::CorePostgresAuthUserResolver;
#[cfg(feature = "auth-http")]
use crate::auth_user::HttpAuthUserResolver;

fn router<T, R>(store: T, auth_user: R) -> Router
where
    T: GetAll + GetById + DeleteOne + SaveOne + UpdateOne + Clone + Send + Sync + 'static,
    R: ResolveAuthUser,
{
    let api_v1 = Router::new()
        .route("/get", get(get::get_all::<T, R>))
        .route("/get/{id}", get(get::get_by_id::<T, R>))
        .route("/delete/{id}", delete(delete::delete_by_id::<T, R>))
        .route("/create", post(create::create::<T, R>))
        .route("/update", patch(update::update::<T, R>));

    Router::new()
        .nest("/api/v1", api_v1)
        .with_state(AppState::new(store, auth_user))
}

#[cfg(feature = "auth-http")]
pub fn basic_actions_router_http<T>(store: T) -> Router
where
    T: GetAll + GetById + DeleteOne + SaveOne + UpdateOne + Clone + Send + Sync + 'static,
{
    router(store, HttpAuthUserResolver)
}

#[cfg(feature = "auth-core-postgres")]
pub fn basic_actions_router_core_postgres<T, A>(store: T, auth_store: A) -> Router
where
    T: GetAll + GetById + DeleteOne + SaveOne + UpdateOne + Clone + Send + Sync + 'static,
    A: auth_dal::users::transactions::get::GetByUniqueId + Clone + Send + Sync + 'static,
{
    router(store, CorePostgresAuthUserResolver(auth_store))
}
