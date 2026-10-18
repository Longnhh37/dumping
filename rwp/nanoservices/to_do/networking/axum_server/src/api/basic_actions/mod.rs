pub mod create;
pub mod delete;
pub mod get;
pub mod update;

use axum::{
    Router,
    routing::{delete, get, patch, post},
};

pub fn router() -> Router {
    let get_routes = Router::new()
        .route("/all", get(get::get_all))
        .route("/{name}", get(get::get_by_name));

    let api_v1 = Router::new()
        .nest("/get", get_routes)
        .route("/delete/{name}", delete(delete::delete_by_name))
        .route("/create", post(create::create))
        .route("update", patch(update::update));


    Router::new().nest("/api/v1", api_v1)
}
