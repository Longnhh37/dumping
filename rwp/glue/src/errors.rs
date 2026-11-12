use serde::{Deserialize, Serialize};
use thiserror::Error;

// ───────────────────────── Types ─────────────────────────

#[derive(Error, Debug, Serialize, Deserialize, PartialEq)]
pub enum NanoServiceErrorStatus {
    #[error("Requested resource was not found")]
    NotFound,

    #[error("You are forbidden to access requested resource")]
    Forbidden,

    #[error("Unknown Internal Error")]
    Unknown,

    #[error("Bad Request")]
    BadRequest,

    #[error("Conflict")]
    Conflict,

    #[error("Unauthorized")]
    Unauthorized,
}

impl NanoServiceErrorStatus {
    pub const fn http_status_code(&self) -> u16 {
        match self {
            Self::NotFound => 404,
            Self::Forbidden => 403,
            Self::Unknown => 500,
            Self::BadRequest => 400,
            Self::Conflict => 409,
            Self::Unauthorized => 401,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Error)]
#[error("{message}")]
pub struct NanoServiceError {
    pub message: String,
    pub status: NanoServiceErrorStatus,
}

impl NanoServiceError {
    pub fn new(message: impl Into<String>, status: NanoServiceErrorStatus) -> Self {
        Self {
            message: message.into(),
            status,
        }
    }
}

// ─────────────────────── Result extensions ───────────────────────

pub trait ResultExt<T> {
    fn eject(self, status: NanoServiceErrorStatus) -> Result<T, NanoServiceError>;
    fn eject_ctx(
        self,
        status: NanoServiceErrorStatus,
        ctx: impl std::fmt::Display,
    ) -> Result<T, NanoServiceError>;
}

impl<T, E: std::fmt::Display> ResultExt<T> for Result<T, E> {
    fn eject(self, status: NanoServiceErrorStatus) -> Result<T, NanoServiceError> {
        self.map_err(|e| NanoServiceError::new(e.to_string(), status))
    }

    fn eject_ctx(
        self,
        status: NanoServiceErrorStatus,
        ctx: impl std::fmt::Display,
    ) -> Result<T, NanoServiceError> {
        self.map_err(|e| NanoServiceError::new(format!("{ctx}: {e}"), status))
    }
}

#[cfg(test)]
mod tests {
    use super::{NanoServiceErrorStatus, ResultExt};

    #[test]
    fn maps_error_statuses_to_http_codes() {
        assert_eq!(NanoServiceErrorStatus::BadRequest.http_status_code(), 400);
        assert_eq!(NanoServiceErrorStatus::NotFound.http_status_code(), 404);
        assert_eq!(NanoServiceErrorStatus::Unauthorized.http_status_code(), 401);
        assert_eq!(NanoServiceErrorStatus::Unknown.http_status_code(), 500);
    }

    #[test]
    fn result_extensions_preserve_context_and_status() {
        let error = Err::<(), _>("storage offline")
            .eject_ctx(NanoServiceErrorStatus::Unknown, "loading items")
            .unwrap_err();

        assert_eq!(error.message, "loading items: storage offline");
        assert_eq!(error.status, NanoServiceErrorStatus::Unknown);
    }
}

// ──────────────────── Framework integrations ────────────────────

#[cfg(feature = "axum")]
mod axum_impl {
    use super::NanoServiceError;
    use axum::{
        Json,
        http::StatusCode,
        response::{IntoResponse, Response},
    };

    impl IntoResponse for NanoServiceError {
        fn into_response(self) -> Response {
            let status = StatusCode::from_u16(self.status.http_status_code())
                .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

            (status, Json(self.to_string())).into_response()
        }
    }
}
