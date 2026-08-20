use axum::{
    Json,
    extract::{ConnectInfo, Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use redis::AsyncCommands;
use serde_json::json;
use std::net::SocketAddr;

use crate::state::AppState;

const MAX_REQUESTS: i64 = 10;
const WINDOW_SECS: u64 = 60;

pub async fn rate_limit(
    State(state): State<AppState>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    request: Request,
    next: Next,
) -> Response {
    let key = format!("rate_limit:{}", addr.ip());
    let mut cache = state.cache.clone();

    let count: i64 = match cache.incr(&key, 1).await {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("redis errpr in rate_limit: {e}");
            return next.run(request).await;
        }
    };

    if count == 1 {
        let _: Result<(), _> = cache.expire(&key, WINDOW_SECS as i64).await;
    }

    if count > MAX_REQUESTS {
        return (
            StatusCode::TOO_MANY_REQUESTS,
            Json(json!({ "error": "Rate limit exceeded. Try again later. "})),
        )
            .into_response();
    }

    next.run(request).await
}
