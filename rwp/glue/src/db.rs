use sqlx::postgres::{PgPool, PgPoolOptions};

use crate::errors::{NanoServiceError, NanoServiceErrorStatus, ResultExt};

pub fn db_error(e: sqlx::Error) -> NanoServiceError {
    match e {
        sqlx::Error::Database(db) if db.is_unique_violation() => {
            NanoServiceError::new("already exist", NanoServiceErrorStatus::Conflict)
        }
        other => {
            tracing::error!(error = %other, "database error");
            NanoServiceError::new("database error", NanoServiceErrorStatus::Unknown)
        }
    }
}

pub fn create_pool(url: &str, max_connections: u32) -> Result<PgPool, NanoServiceError> {
    PgPoolOptions::new()
        .max_connections(max_connections)
        .connect_lazy(url)
        .map_err(db_error)
}

pub fn pool_from_env(prefix: &str) -> Result<PgPool, NanoServiceError> {
    let url_key = format!("{prefix}_DB_URL");
    let url = std::env::var(&url_key).eject_ctx(NanoServiceErrorStatus::Unknown, &url_key)?;

    let max_key = format!("{prefix}_MAX_CONNECTIONS");
    let max = match std::env::var(&max_key) {
        Ok(v) => v
            .parse::<u32>()
            .eject_ctx(NanoServiceErrorStatus::Unknown, &max_key)?,
        Err(_) => 5,
    };
    create_pool(&url, max)
}
