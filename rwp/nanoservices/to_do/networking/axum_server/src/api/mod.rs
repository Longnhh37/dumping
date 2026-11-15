pub mod basic_actions;

use axum::Router;
use to_do_dal::to_do_items::descriptors::SqlxPostgresDescriptor;

pub fn router(store: SqlxPostgresDescriptor) -> Router {
    basic_actions::router(store)
}
