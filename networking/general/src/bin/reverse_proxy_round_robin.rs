use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

use axum::{
    Router,
    body::{self, Body},
    extract::{Request, State},
    http::{HeaderValue, StatusCode, header},
    response::Response,
};

struct Proxy {
    backends: Vec<&'static str>,
    next: AtomicUsize,
    client: reqwest::Client,
}

async fn forward(State(p): State<Arc<Proxy>>, req: Request) -> Result<Response, StatusCode> {
    let i = p.next.fetch_add(1, Ordering::Relaxed) % p.backends.len();
    let path = req.uri().path_and_query().map_or("/", |pq| pq.as_str());
    let url = format!("{}{}", p.backends[i], path);

    let (parts, body) = req.into_parts();
    let bytes = body::to_bytes(body, 1024 * 1024)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let mut headers = parts.headers;
    headers.remove(header::HOST);
    headers.remove(header::CONNECTION);
    headers.insert("x-forwarded-proto", HeaderValue::from_static("http"));

    let upstream = p
        .client
        .request(parts.method, &url)
        .headers(headers)
        .body(bytes)
        .send()
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;

    let status = upstream.status();
    let resp_headers = upstream.headers().clone();

    let mut res = Response::new(Body::from_stream(upstream.bytes_stream()));
    *res.status_mut() = status;
    *res.headers_mut() = resp_headers;
    Ok(res)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let state = Arc::new(Proxy {
        backends: vec!["http://127.0.0.1:3001", "http://127.0.0.1:3002"],
        next: AtomicUsize::new(0),
        client: reqwest::Client::new(),
    });

    let app = Router::new().fallback(forward).with_state(state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app).await
}
