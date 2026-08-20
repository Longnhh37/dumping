pub mod create;
pub mod delete;
pub mod get;
pub mod update;

use actix_web::web::{ServiceConfig, delete, get, post, scope};

pub fn basic_actions_factory(app: &mut ServiceConfig) {
    app.service(
        scope("/api/v1")
            .route("get/all", get().to(get::get_all))
            .route("get/{name}", get().to(get::get_by_name))
            .route("delete/{name}", delete().to(delete::delete_by_name))
            .route("create", post().to(create::create)),
    );
}
