use std::future::Future;

use glue::db::db_error;
use glue::errors::{NanoServiceError, NanoServiceErrorStatus};

use crate::users::descriptors::SqlxPostgresDescriptor;
use crate::users::schema::User;

pub trait GetByEmail {
    fn get_by_email(
        &self,
        email: &str,
    ) -> impl Future<Output = Result<User, NanoServiceError>> + Send;
}

impl GetByEmail for SqlxPostgresDescriptor {
    async fn get_by_email(&self, email: &str) -> Result<User, NanoServiceError> {
        let item = sqlx::query_as::<_, User>(
            "SELECT id, email, password, unique_id FROM users WHERE email = $1",
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(db_error)?;

        item.ok_or_else(|| {
            NanoServiceError::new("user not found", NanoServiceErrorStatus::NotFound)
        })
    }
}
