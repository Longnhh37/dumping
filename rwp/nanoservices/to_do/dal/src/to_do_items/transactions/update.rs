use std::future::Future;

use crate::to_do_items::schema::ToDoItem;
use glue::errors::NanoServiceError;
use glue::db::db_error;


#[cfg(feature = "sqlx-postgres")]
use super::super::descriptors::SqlxPostgresDescriptor;
use glue::errors::NanoServiceErrorStatus;

pub trait UpdateOne {
    fn update_one(
        &self,
        item: ToDoItem,
    ) -> impl Future<Output = Result<ToDoItem, NanoServiceError>> + Send;
}

#[cfg(feature = "sqlx-postgres")]
impl UpdateOne for SqlxPostgresDescriptor {
    async fn update_one(&self, item: ToDoItem) -> Result<ToDoItem, NanoServiceError> {
        sqlx::query_as::<_, ToDoItem>(
            "
        UPDATE to_do_items
        SET status = $2
        WHERE name = $1
        RETURNING name, status",
        )
        .bind(item.name)
        .bind(item.status.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(db_error)?
        .ok_or_else(|| NanoServiceError::new("Item not found", NanoServiceErrorStatus::NotFound))
    }
}
