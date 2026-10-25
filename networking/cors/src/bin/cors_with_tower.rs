use std::time::Duration;

use axum::{
    Router,
    http::{HeaderValue, Method, header},
    routing::get,
};
use tower_http::cors::CorsLayer;

async fn data() -> &'static str {
    "hello from api"
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let cors = CorsLayer::new()
        .allow_origin(HeaderValue::from_static("https://app.example.com"))
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION])
        .allow_credentials(true)
        .max_age(Duration::from_secs(3600));

    let app = Router::new().route("/data", get(data)).layer(cors); // preflight OPTIONS are auto handled

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await
}
