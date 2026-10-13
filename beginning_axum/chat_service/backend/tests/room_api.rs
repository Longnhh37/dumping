use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use backend::{api::state::AppState, app::create_app};
use sea_orm::{DatabaseBackend, MockDatabase};
use tokio::sync::broadcast;
use tower::ServiceExt;

#[tokio::test]
async fn get_rooms_returns_database_rows_as_api_shape() {
    let conn = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([Vec::<backend::entities::room::Model>::new()])
        .into_connection();
    let app = create_app(AppState {
        conn,
        queue: broadcast::channel(10).0,
    });

    let response = app
        .oneshot(Request::builder().uri("/room").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    assert_eq!(body, "[]");
}

#[tokio::test]
async fn send_chat_message_broadcasts_to_subscribers() {
    let conn = MockDatabase::new(DatabaseBackend::Postgres).into_connection();
    let (queue, mut receiver) = broadcast::channel(10);
    let app = create_app(AppState { conn, queue });
    let payload = r#"{"id":0,"timestamp":"2026-01-01T00:00:00Z","sender_id":7,"message":"hello","room_id":3}"#;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/chat/send")
                .header("content-type", "application/json")
                .body(Body::from(payload))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let message = receiver.try_recv().unwrap();
    assert_eq!(message.sender_id, 7);
    assert_eq!(message.room_id, 3);
    assert_eq!(message.message, "hello");
}

#[tokio::test]
async fn send_succeeds_when_there_are_no_live_subscribers() {
    let conn = MockDatabase::new(DatabaseBackend::Postgres).into_connection();
    let (queue, _) = broadcast::channel(10);
    let app = create_app(AppState { conn, queue });
    let payload = r#"{"id":0,"timestamp":"2026-01-01T00:00:00Z","sender_id":7,"message":"hello","room_id":3}"#;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/chat/send")
                .header("content-type", "application/json")
                .body(Body::from(payload))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
