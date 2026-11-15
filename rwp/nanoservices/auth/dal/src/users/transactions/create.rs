use std::future::Future;

use crate::{
    users::descriptors::SqlxPostgresDescriptor,
    users::schema::{NewUser, User},
};
use glue::errors::{NanoServiceError, NanoServiceErrorStatus};

pub trait SaveOne {
    fn save_one(
        &self,
        user: NewUser,
    ) -> impl Future<Output = Result<User, NanoServiceError>> + Send;
}

impl SaveOne for SqlxPostgresDescriptor {
    async fn save_one(&self, user: NewUser) -> Result<User, NanoServiceError> {
        let user = sqlx::query_as::<_, User>(
            "
        INSERT INTO users (email, password, unique_id)
        VALUES ($1, $2, $3)
        RETURNING *",
        )
        .bind(user.email)
        .bind(user.password.to_string())
        .bind(user.unique_id)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| NanoServiceError::new(e.to_string(), NanoServiceErrorStatus::Unknown))?;

        Ok(user)
    }
}
