use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use backend::{api::state::AppState, app::create_app};
use sea_orm::{DatabaseBackend, MockDatabase};
use tokio::sync::broadcast;
use tower::ServiceExt;

fn app() -> axum::Router {
    let conn = MockDatabase::new(DatabaseBackend::Postgres).into_connection();
    create_app(AppState {
        conn,
        queue: broadcast::channel(10).0,
    })
}

#[tokio::test]
async fn unknown_route_serves_frontend_shell() {
    let response = app()
        .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let html = std::str::from_utf8(&body).unwrap();
    assert!(html.contains("<html"));
    assert!(html.contains("id=\"root\""));
}
