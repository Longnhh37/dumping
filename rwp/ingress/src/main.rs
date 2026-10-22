use std::net::{Ipv4Addr, SocketAddr};
use std::path::Path;

use axum::{
    Router,
    http::{StatusCode, Uri, header},
    response::{Html, IntoResponse, Response},
    routing::get,
};
use rust_embed::RustEmbed;
use tower_http::cors::{Any, CorsLayer};

const INDEX_HTML: &str = include_str!("../index.html");

#[derive(RustEmbed)]
#[folder = "../frontend/public"]
struct FrontendAssets;

async fn index() -> Html<&'static str> {
    Html(INDEX_HTML)
}

fn not_found() -> Response {
    (StatusCode::NOT_FOUND, "404 Not Found").into_response()
}

fn serve_asset(path: &str) -> Response {
    let Some(file) = Path::new(path).file_name().and_then(|f| f.to_str()) else {
        return not_found();
    };
    let Some(content) = FrontendAssets::get(file) else {
        return not_found();
    };

    let mime = mime_guess::from_path(file).first_or_octet_stream();

    (
        [
            (header::CONTENT_TYPE, mime.as_ref().to_owned()),
            (header::CACHE_CONTROL, "public, max-age=604800".to_owned()),
        ],
        content.data,
    )
        .into_response()
}

async fn catch_all(uri: Uri) -> Response {
    let path = uri.path();

    if path.starts_with("/api/") {
        return not_found();
    }

    let is_html = mime_guess::from_path(path)
        .first_raw()
        .is_none_or(|m| m == "text/html");

    if is_html {
        index().await.into_response()
    } else {
        serve_asset(path)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(index))
        .fallback(catch_all)
        .layer(cors);

    let addr = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8001));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("listening on http://{addr}");

    axum::serve(listener, app).await?;
    Ok(())
}
