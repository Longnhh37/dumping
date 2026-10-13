use axum::{
    Router,
    routing::{get, post},
};
use tower_http::services::{ServeDir, ServeFile};

use crate::api::{
    chat::{get_chat, send, subscribe},
    chat_room::{delete_room, get_room, post_room, put_room},
    state::AppState,
    user::{delete_user, get_user, post_user, put_user},
};

pub fn create_app(state: AppState) -> Router {
    let root = env!("CARGO_MANIFEST_DIR");
    let static_files = ServeDir::new(format!("{root}/static"))
        .not_found_service(ServeFile::new(format!("{root}/static/index.html")));

    Router::new()
        .nest(
            "/chat",
            Router::new()
                .route("/", get(get_chat))
                .route("/subscribe", get(subscribe))
                .route("/send", post(send)),
        )
        .route(
            "/room",
            get(get_room)
                .post(post_room)
                .put(put_room)
                .delete(delete_room),
        )
        .route(
            "/user",
            get(get_user)
                .post(post_user)
                .put(put_user)
                .delete(delete_user),
        )
        .with_state(state)
        .fallback_service(static_files)
}
