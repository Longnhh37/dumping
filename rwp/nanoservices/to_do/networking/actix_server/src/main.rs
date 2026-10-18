mod api;

use actix_web::{App, HttpServer};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let url = std::env::var("WEB_URL").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("WEB_PORT").unwrap_or_else(|_| "8080".to_string());
    let address = format!("{url}:{port}");

    println!("Serving app on {address}");
    HttpServer::new(|| App::new().configure(api::views_factory))
        .workers(4)
        .bind(&address)?
        .run()
        .await
}
