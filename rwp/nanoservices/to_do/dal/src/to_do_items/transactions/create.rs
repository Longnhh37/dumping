use std::future::Future;

use crate::to_do_items::schema::{NewToDoItem, ToDoItem};
use glue::{db::db_error, errors::NanoServiceError};

#[cfg(feature = "sqlx-postgres")]
use super::super::descriptors::SqlxPostgresDescriptor;

pub trait SaveOne {
    fn save_one(
        &self,
        item: NewToDoItem,
        user_id: &str,
    ) -> impl Future<Output = Result<ToDoItem, NanoServiceError>> + Send;
}

#[cfg(feature = "sqlx-postgres")]
impl SaveOne for SqlxPostgresDescriptor {
    async fn save_one(
        &self,
        item: NewToDoItem,
        user_id: &str,
    ) -> Result<ToDoItem, NanoServiceError> {
        let mut transaction = self.pool.begin().await.map_err(db_error)?;
        let item = sqlx::query_as::<_, ToDoItem>(
            "INSERT INTO to_do_items (title, status) VALUES ($1, $2) RETURNING id, title, status",
        )
        .bind(item.title)
        .bind(item.status.to_string())
        .fetch_one(&mut *transaction)
        .await
        .map_err(db_error)?;

        sqlx::query("INSERT INTO user_connections (user_id, to_do_id) VALUES ($1, $2)")
            .bind(user_id)
            .bind(item.id)
            .execute(&mut *transaction)
            .await
            .map_err(db_error)?;

        transaction.commit().await.map_err(db_error)?;
        Ok(item)
    }
}
