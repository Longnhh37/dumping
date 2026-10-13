mod api;
mod config;
mod entities;

use axum::{
    Router,
    routing::{get, post},
};
use sea_orm::{Database, prelude};
use tokio::sync::broadcast;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

use crate::api::{
    chat::{send, subscribe},
    state::AppState,
};
use crate::config::Config;
use migration::{Migrator, MigratorTrait};

#[tokio::main]
async fn main() {
    let config = Config::from_env();

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let conn = Database::connect(&config.database_url)
        .await
        .expect("Failed to connect to database");

    Migrator::up(&conn, None).await.unwrap();

    let state = AppState {
        conn,
        queue: broadcast::channel(10).0,
    };

    let msg_queue: broadcast::Sender<String> = broadcast::channel(10).0;

    let app = Router::new()
        .nest(
            "/chat",
            Router::new()
                .route("/subscribe", get(subscribe))
                .route("/send", post(send)),
        )
        .with_state(state);

    let addr = format!("0.0.0.0:{}", config.server_port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Server is running on port {}", config.server_port);
    axum::serve(listener, app).await.unwrap();
}
