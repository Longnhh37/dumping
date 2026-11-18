use tokio::net::TcpListener;

use auth_axum_server::api;
use auth_dal::{migrations::run_migrations, users::descriptors::SqlxPostgresDescriptor};
use glue::db::pool_from_env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let host = std::env::var("AUTH_WEB_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("AUTH_WEB_PORT").unwrap_or_else(|_| "8081".to_string());
    let address = format!("{host}:{port}");

    let pool = pool_from_env("AUTH")?;
    run_migrations(&pool).await?;

    let listener = TcpListener::bind(&address).await?;
    let app = api::router(api::AppState::new(SqlxPostgresDescriptor::new(pool)));

    println!("Serving app on {address}");
    axum::serve(listener, app).await?;
    Ok(())
}
