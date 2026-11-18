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

use crate::api::AppState;

pub fn basic_actions_router<T>(store: T) -> Router
where
    T: GetAll + GetById + DeleteOne + SaveOne + UpdateOne + Clone + Send + Sync + 'static,
{
    let api_v1 = Router::new()
        .route("/get", get(get::get_all::<T>))
        .route("/get/{id}", get(get::get_by_id::<T>))
        .route("/delete/{id}", delete(delete::delete_by_id::<T>))
        .route("/create", post(create::create::<T>))
        .route("/update", patch(update::update::<T>));

    Router::new()
        .nest("/api/v1", api_v1)
        .with_state(AppState { store })
}
