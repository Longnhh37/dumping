use axum::{
    Router,
    extract::FromRequestParts,
    http::{StatusCode, header, request::Parts},
    routing::get,
};
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode};
use serde::Deserialize;

// production: read from .env
const SECRET: &[u8] = b"replace-this-with-a-randomized-secret-thats-longer-than-32-byte";

#[derive(Debug, Deserialize)]
struct AuthUser {
    sub: String,
    role: String,
}

impl<S: Send + Sync> FromRequestParts<S> for AuthUser {
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let token = parts
            .headers
            .get(header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .ok_or(StatusCode::UNAUTHORIZED)?;

        let validation = Validation::new(Algorithm::HS256);
        decode::<AuthUser>(token, &DecodingKey::from_secret(SECRET), &validation)
            .map(|d| d.claims)
            .map_err(|_| StatusCode::UNAUTHORIZED)
    }
}

async fn me(user: AuthUser) -> String {
    format!("hello {} ({})", user.sub, user.role)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = Router::new().route("/me", get(me));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await
}
