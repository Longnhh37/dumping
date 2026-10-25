use axum::http::{HeaderValue, Method, header};
use tower_http::cors::{AllowOrigin, CorsLayer};

fn cors_layer() -> CorsLayer {
    let allowed = ["https://app.example.com", "https://admin.example.com"];

    CorsLayer::new()
        .allow_origin(AllowOrigin::predicate(
            move |origin: &HeaderValue, _parts| {
                allowed.iter().any(|a| origin.as_bytes() == a.as_bytes())
            },
        ))
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::CONTENT_TYPE])
        .allow_credentials(true)
}

fn main() {}
