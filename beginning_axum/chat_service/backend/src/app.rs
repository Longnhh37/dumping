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
    let static_files =
        ServeDir::new("static").not_found_service(ServeFile::new("static/index.html"));

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

#[cfg(test)]
mod tests {
    use crate::config::Config;

    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use sea_orm::Database;
    use tokio::sync::broadcast;
    use tower::ServiceExt;
    use tower_http::set_header::request;

    #[tokio::test]
    async fn test_create_room_endpoint() {
        // 1. test env
        let config = Config::from_env();
        let conn = Database::connect(&config.database_url).await.unwrap();
        let state = AppState {
            conn,
            queue: broadcast::channel(10).0,
        };
        let app = create_app(state);

        // 2. create a mock http post request object
        let request = Request::builder()
            .uri("/room")
            .method("POST")
            .header("Content-Type", "application/json")
            .body(Body::from(r#"{"participants: ["alice", "bob"]}"#))
            .unwrap();

        // 3. send the request directly to the router (no actual server requried)
        let response = app.oneshot(request).await.unwrap();

        // 4. verify response
        assert_eq!(response.status(), StatusCode::OK);
    }
}
