use std::future::Future;

use crate::to_do_items::schema::ToDoItem;
use glue::db::db_error;
use glue::errors::NanoServiceError;

#[cfg(feature = "sqlx-postgres")]
use super::super::descriptors::SqlxPostgresDescriptor;
use glue::errors::NanoServiceErrorStatus;

pub trait UpdateOne {
    fn update_one(
        &self,
        item: ToDoItem,
        user_id: &str,
    ) -> impl Future<Output = Result<ToDoItem, NanoServiceError>> + Send;
}

#[cfg(feature = "sqlx-postgres")]
impl UpdateOne for SqlxPostgresDescriptor {
    async fn update_one(
        &self,
        item: ToDoItem,
        user_id: &str,
    ) -> Result<ToDoItem, NanoServiceError> {
        sqlx::query_as::<_, ToDoItem>(
            "
        UPDATE to_do_items
        SET status = $3
        WHERE id = $1
          AND EXISTS (SELECT 1 FROM user_connections WHERE to_do_id = to_do_items.id AND user_id = $2)
        RETURNING id, title, status",
        )
        .bind(item.id)
        .bind(user_id)
        .bind(item.status.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(db_error)?
        .ok_or_else(|| NanoServiceError::new("Item not found", NanoServiceErrorStatus::NotFound))
    }
}
