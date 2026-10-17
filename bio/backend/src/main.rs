use std::env;

use axum::{Form, Router, response::Redirect, routing::post};
use tower_http::services::ServeDir;

const DEFAULT_NAME: &str = "Anonymous User";
const MAX_NAME_CHARS: usize = 50;

#[derive(serde::Deserialize)]
struct NameForm {
    name: Option<String>,
}

async fn submit(Form(form): Form<NameForm>) -> Redirect {
    let name: String = form
        .name
        .as_deref()
        .map(str::trim)
        .filter(|n| !n.is_empty())
        .unwrap_or(DEFAULT_NAME)
        .chars()
        .take(MAX_NAME_CHARS)
        .collect();

    let encoded = urlencoding::encode(&name);
    Redirect::to(&format!("/linktree.html?name={encoded}"))
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let frontend_dir = env::var("FRONTEND_DIR").unwrap_or_else(|_| "../frontend".to_owned());

    let app = Router::new()
        .route("/submit", post(submit))
        .fallback_service(ServeDir::new(frontend_dir));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
    println!("Listening on http://{}", listener.local_addr()?);

    axum::serve(listener, app).await
}
