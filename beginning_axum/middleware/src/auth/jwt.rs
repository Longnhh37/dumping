use std::{env, sync::LazyLock};

use axum::{
    body::Body,
    http::{Request, StatusCode, header},
    middleware::Next,
    response::Response,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use tracing::{debug, error};

use super::app_error::AppError;

pub static SECRET_KEY: LazyLock<String> =
    LazyLock::new(|| env::var("SECRET_KEY").expect("SECRET_KEY must be set"));

static ENCODING_KEY: LazyLock<EncodingKey> =
    LazyLock::new(|| EncodingKey::from_secret(SECRET_KEY.as_bytes()));

static DECODING_KEY: LazyLock<DecodingKey> =
    LazyLock::new(|| DecodingKey::from_secret(SECRET_KEY.as_bytes()));

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub exp: u64,
    pub username: String,
}

pub fn create_token(username: String) -> Result<String, AppError> {
    let exp = (Utc::now() + Duration::hours(1)).timestamp() as u64;
    let claims = Claims { exp, username };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(SECRET_KEY.as_bytes()),
    )
    .map_err(|err| {
        error!("error creating token: {err:?}");
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "There was an error, please try again later",
        )
    })
}

pub fn validate_token(token: &str) -> Result<Claims, AppError> {
    let token = token.strip_prefix("Bearer ").unwrap_or(token);
    let validation = Validation::new(Algorithm::HS256);
    let data = decode::<Claims>(token, &DECODING_KEY, &validation)?;
    Ok(data.claims)
}

pub async fn authenticate(req: Request<Body>, next: Next) -> Result<Response, AppError> {
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .ok_or_else(|| AppError::new(StatusCode::UNAUTHORIZED, "not authenticated"))?
        .to_str()
        .map_err(|err| {
            error!("Error reading Authorization header: {err:?}");
            AppError::new(StatusCode::BAD_REQUEST, "Invalid authorization header")
        })?;

    let claims = validate_token(token)?;
    debug!("Authenticated user: {}", claims.username);

    Ok(next.run(req).await)
}
