use std::future::Future;

use crate::to_do_items::schema::ToDoItem;
use glue::db::db_error;
use glue::errors::{NanoServiceError, NanoServiceErrorStatus};

#[cfg(feature = "sqlx-postgres")]
use super::super::descriptors::SqlxPostgresDescriptor;

pub trait DeleteOne {
    fn delete_one(
        &self,
        name: &str,
    ) -> impl Future<Output = Result<ToDoItem, NanoServiceError>> + Send;
}

#[cfg(feature = "sqlx-postgres")]
impl DeleteOne for SqlxPostgresDescriptor {
    async fn delete_one(&self, name: &str) -> Result<ToDoItem, NanoServiceError> {
        sqlx::query_as::<_, ToDoItem>(
            "
        DELETE FROM to_do_items
        WHERE name = $1
        RETURNING name, status",
        )
        .bind(name)
        .fetch_optional(&self.pool)
        .await
        .map_err(db_error)?
        .ok_or_else(|| NanoServiceError::new("Item not found", NanoServiceErrorStatus::NotFound))
    }
}
