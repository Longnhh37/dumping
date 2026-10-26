use axum::{
    Router,
    extract::Request,
    http::{Method, StatusCode},
    middleware::{self, Next},
    response::Response,
    routing::post,
};

async fn csrf_guard(req: Request, next: Next) -> Result<Response, StatusCode> {
    let is_write = !matches!(*req.method(), Method::GET | Method::HEAD | Method::OPTIONS);

    if is_write {
        let site = req
            .headers()
            .get("sec-fetch-site")
            .and_then(|v| v.to_str().ok());

        match site {
            // same-origin: JS của chính site; none: user thao tác trực tiếp
            Some("same-origin") | Some("none") => {}
            // Không có header: client không phải browser (curl, mobile app), cho qua
            None => {}
            // same-site / cross-site: từ chối
            Some(_) => return Err(StatusCode::FORBIDDEN),
        }
    }
    Ok(next.run(req).await)
}

async fn transfer() -> &'static str {
    "done"
}

fn app() -> Router {
    Router::new()
        .route("/transfer", post(transfer))
        .layer(middleware::from_fn(csrf_guard))
}

fn main() {}
