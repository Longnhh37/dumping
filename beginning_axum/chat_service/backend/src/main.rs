mod api;
mod app;
mod config;
mod entities;

use sea_orm::Database;
use tokio::sync::broadcast;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

use crate::api::state::AppState;
use crate::app::create_app;
use crate::config::Config;
use migration::{Migrator, MigratorTrait};

#[tokio::main]
async fn main() -> std::io::Result<()> {
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
    let app = create_app(state);

    let addr = format!("0.0.0.0:{}", config.server_port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Server is running on port {}", config.server_port);
    axum::serve(listener, app).await
}
