use axum::{
    Router,
    extract::{FromRef, State},
    routing::get,
};

#[derive(FromRef, Clone)]
struct AppState {
    auth_token: String,
    current_users: i32,
}

async fn token(State(auth_token): State<String>) -> String {
    format!("Token: {}", auth_token)
}
async fn users(State(current_users): State<i32>) -> String {
    format!("Current user: {}", current_users)
}

#[tokio::main]
async fn main() {
    let state = AppState {
        auth_token: "auth_tok".to_string(),
        current_users: 3,
    };
    let app = Router::new()
        .route("/users", get(users))
        .route("/token", get(token))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
