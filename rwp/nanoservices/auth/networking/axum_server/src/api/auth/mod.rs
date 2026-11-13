pub mod logout;
pub mod login;

use axum::Router;

pub fn auth_factory(app: Router) -> Router {
    app
}
