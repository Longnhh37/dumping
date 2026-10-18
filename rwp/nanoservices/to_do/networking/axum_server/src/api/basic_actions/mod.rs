pub mod create;
pub mod delete;
pub mod get;
pub mod update;

use axum::{Router, routing::{delete, get, post}};

pub fn router() -> Router {
    Router::new()
        .route("/api/v1/get/all", get(get::get_all))
        .route("/api/v1/get/{name}", get(get::get_by_name))
        .route("/api/v1/delete/{name}", delete(delete::delete_by_name))
        .route("/api/v1/create", post(create::create))
}
