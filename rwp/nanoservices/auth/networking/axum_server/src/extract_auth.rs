use axum::http::{Request, header};
use base64::{Engine, engine::general_purpose};

use glue::errors::{NanoServiceError, NanoServiceErrorStatus};

#[derive(Debug)]
pub struct Credentials {
    pub email: String,
    pub password: String,
}

pub fn extract_credentials<B>(req: &Request<B>) -> Result<Credentials, NanoServiceError> {
    let invalid = || {
        NanoServiceError::new("Invalid credentials", NanoServiceErrorStatus::Unauthorized)
    };

    let header_value = req.headers().get(header::AUTHORIZATION).ok_or_else(|| {
        NanoServiceError::new("No credentials provided", NanoServiceErrorStatus::Unauthorized)
    })?;

    let encoded = header_value.to_str().map_err(|_| invalid())?;
    let base64_credentials = encoded.strip_prefix("Basic ").ok_or_else(invalid)?;

    let decoded = general_purpose::STANDARD
        .decode(base64_credentials)
        .map_err(|_| invalid())?;
    let credentials = String::from_utf8(decoded).map_err(|_| invalid())?;

    let (email, password) = credentials.split_once(':').ok_or_else(invalid)?;

    Ok(Credentials {
        email: email.to_owned(),
        password: password.to_owned(),
    })
}
