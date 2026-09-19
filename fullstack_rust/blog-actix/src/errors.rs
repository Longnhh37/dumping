use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use diesel::result::{DatabaseErrorKind::UniqueViolation, Error as DieselError};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("database error: {0}")]
    Database(DieselError),

    #[error("database pool error: {0}")]
    DatabasePool(String),

    #[error("operation canceled")]
    OperationCanceled,

    #[error("record already exists")]
    RecordAlreadyExists,

    #[error("record not found")]
    RecordNotFound,

    #[error("bad request: {0}")]
    BadRequest(String),
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::DatabasePool(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::OperationCanceled => StatusCode::INTERNAL_SERVER_ERROR,
            Self::RecordAlreadyExists => StatusCode::CONFLICT,
            Self::RecordNotFound => StatusCode::NOT_FOUND,
            Self::BadRequest(_) => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }
}

impl From<DieselError> for AppError {
    fn from(value: DieselError) -> Self {
        match value {
            DieselError::DatabaseError(UniqueViolation, _) => Self::RecordAlreadyExists,
            DieselError::NotFound => Self::RecordNotFound,
            other => Self::Database(other),
        }
    }
}
