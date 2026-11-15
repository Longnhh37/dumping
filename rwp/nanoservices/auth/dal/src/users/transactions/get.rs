use std::future::Future;

use glue::errors::{NanoServiceError, NanoServiceErrorStatus};

use crate::connections::sqlx_postgres::SQLX_POSTGRES_POOL;
use crate::users::descriptors::SqlxPostgresDescriptor;
use crate::users::schema::User;

pub trait GetByEmail {
    fn get_by_email(email: String) -> impl Future<Output = Result<User, NanoServiceError>> + Send;
}

impl GetByEmail for SqlxPostgresDescriptor {
    async fn get_by_email(email: String) -> Result<User, NanoServiceError> {
        let item = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
            .bind(email)
            .fetch_optional(&*SQLX_POSTGRES_POOL)
            .await
            .map_err(|e| NanoServiceError::new(e.to_string(), NanoServiceErrorStatus::Unknown))?;

        item.ok_or_else(|| {
            NanoServiceError::new("user not found", NanoServiceErrorStatus::NotFound)
        })
    }
}
