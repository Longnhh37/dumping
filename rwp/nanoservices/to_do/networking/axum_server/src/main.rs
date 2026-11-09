use axum::Router;
use tokio::net::TcpListener;

use to_do_axum_server::api;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let url = std::env::var("WEB_URL").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("WEB_PORT").unwrap_or_else(|_| "8080".to_string());
    let address = format!("{url}:{port}");

    let listener = TcpListener::bind(&address).await?;
    let app = Router::new().merge(api::router());

    println!("Serving app on {address}");
    axum::serve(listener, app).await
}
