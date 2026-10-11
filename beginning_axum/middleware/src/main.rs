mod auth;

use std::{sync::LazyLock, time::Duration};

use axum::{Router, http::StatusCode, middleware, routing::get};
use tower_http::{compression::CompressionLayer, timeout::TimeoutLayer, trace::TraceLayer};
use tracing::info;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

use crate::auth::jwt::{SECRET_KEY, authenticate};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    LazyLock::force(&SECRET_KEY);

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    info!("Starting app...");
    let app = Router::new()
        .route("/", get("OK"))
        .route_layer(middleware::from_fn(authenticate))
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_millis(3000),
        ))
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new());

    info!("Starting tcp listner...");
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    info!("Starting server...");
    axum::serve(listener, app).await.unwrap();
}
