pub mod create;

use axum::{
    Router,
    routing::{delete, get, patch, post},
};

use auth_dal::users::descriptors::SqlxPostgresDescriptor;

pub fn users_router() -> Router {
        Router::new().route("/users/create", post(create::create::<SqlxPostgresDescriptor>))
}
