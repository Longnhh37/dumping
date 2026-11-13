pub mod auth;
pub mod users;

use axum::Router;

pub fn router() -> Router {
    let app = Router::new();
    let app = users::users_factory(app);
    let app = auth::auth_factory(app);
    app
}
