use tokio::net::TcpListener;

use auth_axum_server::api;
use auth_dal::migrations::run_migrations;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let host = std::env::var("AUTH_WEB_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("AUTH_WEB_PORT").unwrap_or_else(|_| "8081".to_string());
    let address = format!("{host}:{port}");

    run_migrations().await.expect("failed to run migrations");

    let listener = TcpListener::bind(&address).await?;
    let app = api::router();

    println!("Serving app on {address}");
    axum::serve(listener, app).await
}
