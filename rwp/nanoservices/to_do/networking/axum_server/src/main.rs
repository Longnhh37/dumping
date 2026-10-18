mod api;

use axum::Router;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    axum::serve(listener, Router::new().merge(api::router())).await
}
