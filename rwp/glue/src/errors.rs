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

#[macro_export]
macro_rules! safe_eject {
    ($e:expr, $status:expr) => {
        $e.map_err(|x| $crate::NanoServiceError::new(x.to_string(), $status))
    };
    ($e:expr, $status:expr, $ctx:expr) => {
        $e.map_err(|x| $crate::NanoServiceError::new(format!("{}: {x}", $ctx), $status))
    };
}

// ──────────────────── Framework integrations ────────────────────

#[cfg(feature = "actix")]
mod actix_impl {
    use super::NanoServiceError;
    use actix_web::{HttpResponse, error::ResponseError, http::StatusCode};

    impl ResponseError for NanoServiceError {
        fn status_code(&self) -> StatusCode {
            StatusCode::from_u16(self.status.http_status_code())
                .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
        }

        fn error_response(&self) -> HttpResponse {
            HttpResponse::build(self.status_code()).json(self.to_string())
        }
    }
}

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
