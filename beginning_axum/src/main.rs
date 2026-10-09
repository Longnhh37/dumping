use axum::{
    Router,
    extract::Query,
    routing::{delete, get, post, put},
};
use serde::Deserialize;

#[derive(Deserialize)]
struct User {
    id: i32,
    name: String,
}

#[tokio::main]
async fn main() {
    let user_routes = Router::new()
        .route("/", get(|| async move { "user" }))
        .route("/login", get(|| async move { "login" }));

    let team_routes = Router::new().route("/", post(|| async move { "teams" }));

    let api_routes = Router::new()
        .nest("/users", user_routes)
        .nest("/teams", team_routes);

    let app = Router::new().route(
        "/",
        get(|| async move { "Welcome to Axum!" })
            .post(|| async move { "Post something" })
            .put(|| async move { "Updating..." })
            .delete(|| async move { "Deleting..." }),
    );
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
