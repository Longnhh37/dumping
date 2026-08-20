mod api;

use actix_web::{App, HttpServer};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(api::views_factory))
        .workers(4)
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
