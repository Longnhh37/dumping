mod api;

use axum::Router;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    let app = Router::new().merge(api::router());

    println!("Serving app on 127.0.0.1:8080");
    axum::serve(listener, app).await
}
