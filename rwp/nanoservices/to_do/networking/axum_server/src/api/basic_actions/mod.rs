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
    get::{GetAll, GetByName},
    update::UpdateOne,
};

use to_do_dal::to_do_items::descriptors::SqlxPostgresDescriptor;

pub fn basic_actions_router<T>() -> Router
where
    T: GetAll + GetByName + DeleteOne + SaveOne + UpdateOne + 'static,
{
    let get_routes = Router::new()
        .route("/all", get(get::get_all::<T>))
        .route("/{name}", get(get::get_by_name::<T>));

    let api_v1 = Router::new()
        .nest("/get", get_routes)
        .route("/delete/{name}", delete(delete::delete_by_name::<T>))
        .route("/create", post(create::create::<T>))
        .route("/update", patch(update::update::<T>));

    Router::new().nest("/api/v1", api_v1)
}

pub fn router() -> Router {
    basic_actions_router::<SqlxPostgresDescriptor>()
}
