use std::future::Future;

use crate::{
    users::descriptors::SqlxPostgresDescriptor,
    users::schema::{NewUser, User},
};
use glue::db::db_error;
use glue::errors::NanoServiceError;

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
        RETURNING id, email, password, unique_id",
        )
        .bind(user.email)
        .bind(user.password_hash)
        .bind(user.unique_id)
        .fetch_one(&self.pool)
        .await
        .map_err(db_error)?;

        Ok(user)
    }
}
