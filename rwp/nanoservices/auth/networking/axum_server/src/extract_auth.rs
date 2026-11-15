use axum::http::{Request, header};
use base64::{Engine, engine::general_purpose};

use glue::errors::{NanoServiceError, NanoServiceErrorStatus};

#[derive(Debug)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}

pub fn extract_credentials<B>(req: &Request<B>) -> Result<Credentials, NanoServiceError> {
    let header_value = req.headers().get(header::AUTHORIZATION).ok_or_else(|| {
        NanoServiceError::new(
            "No credentials provided",
            NanoServiceErrorStatus::Unauthorized,
        )
    })?;

    let encoded = header_value.to_str().map_err(|_| {
        NanoServiceError::new("Invalid credentials", NanoServiceErrorStatus::Unauthorized)
    })?;

    if !encoded.starts_with("Basic ") {
        return Err(NanoServiceError::new(
            "Invalid credentials",
            NanoServiceErrorStatus::Unauthorized,
        ));
    }

    let base64_credentials = &encoded[6..];
    let decoded = general_purpose::STANDARD
        .decode(base64_credentials)
        .map_err(|e| NanoServiceError::new(e.to_string(), NanoServiceErrorStatus::Unauthorized))?;
    let credentials = String::from_utf8(decoded)
        .map_err(|e| NanoServiceError::new(e.to_string(), NanoServiceErrorStatus::Unauthorized))?;

    let mut split = credentials.splitn(2, ':');
    match (split.next(), split.next()) {
        (Some(email), Some(password)) => Ok(Credentials {
            email: email.to_owned(),
            password: password.to_owned(),
        }),
        _ => Err(NanoServiceError::new(
            "Invalid credentials",
            NanoServiceErrorStatus::Unauthorized,
        )),
    }
}
