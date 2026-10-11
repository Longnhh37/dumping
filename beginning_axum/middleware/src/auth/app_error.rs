use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use jsonwebtoken::errors::{Error as JwtError, ErrorKind};
use serde_json::json;

pub struct AppError {
    code: StatusCode,
    message: String,
}

impl AppError {
    pub fn new(code: StatusCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (self.code, Json(json!({ "error": self.message }))).into_response()
    }
}

impl From<JwtError> for AppError {
    fn from(err: JwtError) -> Self {
        match err.kind() {
            ErrorKind::ExpiredSignature => AppError::new(StatusCode::UNAUTHORIZED, "token expired"),
            ErrorKind::InvalidToken
            | ErrorKind::InvalidSignature
            | ErrorKind::InvalidAlgorithm
            | ErrorKind::ImmatureSignature
            | ErrorKind::Base64(_)
            | ErrorKind::Json(_)
            | ErrorKind::Utf8(_) => AppError::new(StatusCode::UNAUTHORIZED, "invalid token"),
            _ => AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "internal server error"),
        }
    }
}
