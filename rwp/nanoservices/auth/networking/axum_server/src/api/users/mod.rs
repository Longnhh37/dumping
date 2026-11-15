pub mod create;

use auth_dal::users::descriptors::SqlxPostgresDescriptor;
use axum::{
    Router,
    routing::{delete, get, patch, post},
};

pub fn users_factory(app: Router) -> Router {
    app
        .route(", method_router)
}
