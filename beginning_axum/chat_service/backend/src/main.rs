mod api;

use axum::{
    Router,
    routing::{get, post},
};
use tokio::sync::broadcast;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

use api::chat::{send, subscribe};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let msg_queue: broadcast::Sender<String> = broadcast::channel(10).0;

    let app = Router::new()
        .nest(
            "/chat",
            Router::new()
                .route("/subscribe", get(subscribe))
                .route("/send", get(send)),
        )
        .with_state(msg_queue);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server is running on port 3000");
    axum::serve(listener, app).await.unwrap();
}
