use std::future::Future;

use crate::to_do_items::schema::ToDoItem;
use glue::{db::db_error, errors::NanoServiceError};

#[cfg(feature = "sqlx-postgres")]
use super::super::descriptors::SqlxPostgresDescriptor;

pub trait SaveOne {
    fn save_one(
        &self,
        item: ToDoItem,
    ) -> impl Future<Output = Result<ToDoItem, NanoServiceError>> + Send;
}

#[cfg(feature = "sqlx-postgres")]
impl SaveOne for SqlxPostgresDescriptor {
    async fn save_one(&self, item: ToDoItem) -> Result<ToDoItem, NanoServiceError> {
        let item = sqlx::query_as::<_, ToDoItem>(
            "
    INSERT INTO to_do_items (name, status)
    VALUES ($1, $2)
    RETURNING name, status",
        )
        .bind(item.name)
        .bind(item.status.to_string())
        .fetch_one(&self.pool)
        .await
        .map_err(db_error)?;

        Ok(item)
    }
}
