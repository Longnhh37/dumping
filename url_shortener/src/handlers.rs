use anyhow::Result;
use axum::{
    Json,
    extract::{Path, State},
    response::Redirect,
};
use redis::AsyncCommands;

use crate::{
    error::AppError,
    models::{ShortenRequest, ShortenResponse, UrlRecord},
    state::AppState,
};

const ALPHABET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
const CACHE_TTL_SECS: u64 = 3600;

fn to_base62(mut n: i64) -> String {
    if n == 0 {
        return "0".to_string();
    }
    let mut res = Vec::new();
    while n > 0 {
        res.push(ALPHABET[(n % 62) as usize]);
        n /= 62;
    }
    res.reverse();
    String::from_utf8(res).unwrap()
}

fn from_base62(s: &str) -> Result<i64, AppError> {
    let mut n = 0i64;
    for c in s.chars() {
        let digit = ALPHABET
            .iter()
            .position(|&x| x == c as u8)
            .ok_or(AppError::NotFound)?;
        n = n * 62 + digit as i64;
    }

    Ok(n)
}

pub async fn shorten(
    State(state): State<AppState>,
    Json(payload): Json<ShortenRequest>,
) -> Result<Json<ShortenResponse>, AppError> {
    if !payload.url.starts_with("http://") && !payload.url.starts_with("https://") {
        return Err(AppError::InvalidUrl(payload.url));
    }

    let id: i64 = sqlx::query_scalar("INSERT INTO urls (long_url) VALUES ($1) RETURNING id")
        .bind(&payload.url)
        .fetch_one(&state.db)
        .await?;

    let short_code = to_base62(id);

    Ok(Json(ShortenResponse {
        short_code,
        long_url: payload.url,
    }))
}

pub async fn redirect(
    State(mut state): State<AppState>,
    Path(short_code): Path<String>,
) -> Result<Redirect, AppError> {
    let cache_key = format!("short:{short_code}");

    // 1. cache hit
    if let Ok(cached_url) = state.cache.get::<_, String>(&cache_key).await {
        return Ok(Redirect::temporary(&cached_url));
    }

    // 2. cache miss
    let id = from_base62(&short_code)?;

    let record: UrlRecord =
        sqlx::query_as("SELECT long_url FROM urls WHERE id = $1")
            .bind(id)
            .fetch_optional(&state.db)
            .await?
            .ok_or(AppError::NotFound)?;

    // 3. insert to cache
    let _: Result<(), _> = state
        .cache
        .set_ex(&cache_key, &record.long_url, CACHE_TTL_SECS)
        .await?;

    Ok(Redirect::temporary(&record.long_url))
}

pub async fn health() -> &'static str {
    "ok"
}
