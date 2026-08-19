mod error;
mod handlers;
mod models;
mod state;

use axum::{
    Router,
    routing::{get, post},
};
use state::AppState;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let database_url = env::var("DATABASE_URL")?;
    let redis_url = env::var("REDIS_URL")?;

    let state = AppState::new(&database_url, &redis_url).await?;

    let app = Router::new()
        .route("/health", get(handlers::health))
        .route("/shorten", post(handlers::shorten))
        .route("/{short_code}", get(handlers::redirect))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    tracing::info!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
