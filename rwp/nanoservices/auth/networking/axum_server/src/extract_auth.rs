use axum::{
    extract::FromRequestParts,
    http::{HeaderMap, header, request::Parts},
};
use base64::{Engine, engine::general_purpose};

use glue::errors::{NanoServiceError, NanoServiceErrorStatus};

#[derive(Debug)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}

fn invalid() -> NanoServiceError {
    NanoServiceError::new("Invalid credentials", NanoServiceErrorStatus::Unauthorized)
}

pub fn parse_basic(headers: &HeaderMap) -> Result<Credentials, NanoServiceError> {
    let value = headers.get(header::AUTHORIZATION).ok_or_else(|| {
        NanoServiceError::new(
            "No credentials provided",
            NanoServiceErrorStatus::Unauthorized,
        )
    })?;
    let encoded = value.to_str().map_err(|_| invalid())?;
    let b64 = encoded.strip_prefix("Basic ").ok_or_else(invalid)?;
    let decoded = general_purpose::STANDARD
        .decode(b64)
        .map_err(|_| invalid())?;
    let text = String::from_utf8(decoded).map_err(|_| invalid())?;
    let (email, password) = text.split_once(':').ok_or_else(invalid)?;
    Ok(Credentials {
        email: email.to_owned(),
        password: password.to_owned(),
    })
}

impl<S: Send + Sync> FromRequestParts<S> for Credentials {
    type Rejection = NanoServiceError;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        parse_basic(&parts.headers)
    }
}
