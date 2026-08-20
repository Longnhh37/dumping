mod error;
mod handlers;
mod models;
mod rate_limit;
mod state;

use axum::{
    Router, middleware,
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
        .route(
            "/shorten",
            post(handlers::shorten).route_layer(middleware::from_fn_with_state(
                state.clone(),
                rate_limit::rate_limit,
            )),
        )
        .route("/{short_code}", get(handlers::redirect))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    tracing::info!("listening on {}", listener.local_addr()?);
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await?;

    Ok(())
}
