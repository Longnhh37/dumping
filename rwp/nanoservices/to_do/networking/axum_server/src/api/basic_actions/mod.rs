pub mod create;
pub mod delete;
pub mod get;
pub mod update;

use axum::{
    Router,
    routing::{delete, get, patch, post},
};

use to_do_dal::to_do_items::descriptors::SqlxPostgresDescriptor;

pub fn basic_actions_router() -> Router {
    let get_routes = Router::new()
        .route("/all", get(get::get_all::<SqlxPostgresDescriptor>))
        .route("/{name}", get(get::get_by_name::<SqlxPostgresDescriptor>));

    let api_v1 = Router::new()
        .nest("/get", get_routes)
        .route(
            "/delete/{name}",
            delete(delete::delete_by_name::<SqlxPostgresDescriptor>),
        )
        .route("/create", post(create::create::<SqlxPostgresDescriptor>))
        .route("/update", patch(update::update::<SqlxPostgresDescriptor>));

    Router::new().nest("/api/v1", api_v1)
}
