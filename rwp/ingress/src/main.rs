use std::{ffi::OsStr, net::Ipv4Addr, path::Path};

use axum::{
    Router,
    http::{StatusCode, Uri, header},
    response::{Html, IntoResponse, Response},
    routing::get,
};
use rust_embed::RustEmbed;
use to_do_dal::migrations::run_migrations;
use tower_http::cors::{Any, CorsLayer};

const INDEX_HTML: &str = include_str!("../index.html");
const NOT_FOUND: (StatusCode, &str) = (StatusCode::NOT_FOUND, "404 Not Found");

#[derive(RustEmbed)]
#[folder = "../frontend/public"]
struct FrontendAssets;

async fn index() -> Html<&'static str> {
    Html(INDEX_HTML)
}

fn serve_frontend_asset(path: &str) -> Response {
    let Some(file) = Path::new(path).file_name().and_then(OsStr::to_str) else {
        return NOT_FOUND.into_response();
    };
    let Some(asset) = FrontendAssets::get(file) else {
        return NOT_FOUND.into_response();
    };

    let mime = mime_guess::from_path(file).first_or_octet_stream().to_string();

    (
        [
            (header::CONTENT_TYPE, mime),
            (header::CACHE_CONTROL, "public, max-age=604800".to_owned()),
        ],
        asset.data,
    )
        .into_response()
}

async fn catch_all(uri: Uri) -> Response {
    let path = uri.path();

    if path.contains("/api/") {
        return (StatusCode::NOT_FOUND, "Not Found").into_response();
    }
    if path.contains("frontend/public") {
        return serve_frontend_asset(path);
    }

    let wants_html = mime_guess::from_path(path)
        .first_raw()
        .is_none_or(|mime| mime.contains("text/html"));

    if wants_html {
        index().await.into_response()
    } else {
        serve_frontend_asset(path)
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    run_migrations().await.expect("Failed to run migrations");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(index))
        .fallback(catch_all)
        .layer(cors);

    let listener = tokio::net::TcpListener::bind((Ipv4Addr::UNSPECIFIED, 8001)).await?;
    axum::serve(listener, app).await
}
