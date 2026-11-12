pub mod basic_actions;

use axum::Router;

pub fn router() -> Router {
    basic_actions::router()
}
