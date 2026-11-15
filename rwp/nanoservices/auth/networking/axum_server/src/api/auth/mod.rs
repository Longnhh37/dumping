pub mod login;
pub mod logout;

use axum::Router;

use auth_core::api::users::create;
use auth_dal::users::descriptors::SqlxPostgresDescriptor;

pub fn auth_router() -> Router {
    Router::new()
}
