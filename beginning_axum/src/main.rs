use axum::{
    Router,
    routing::{delete, get, post, put},
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async move { "Welcome to Axum!" }))
        .route("/", post(|| async move { "Post something" }))
        .route("/", put(|| async move { "Updating..." }))
        .route("/", delete(|| async move { "Deleting..." }));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
