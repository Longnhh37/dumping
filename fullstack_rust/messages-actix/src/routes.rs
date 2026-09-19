use crate::handlers;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(handlers::index))
        .route("/send", web::post().to(handlers::post))
        .route("/clear", web::post().to(handlers::clear))
        .route("/lookup/{index}", web::get().to(handlers::lookup));
}
