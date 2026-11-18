use std::future::Future;

use glue::db::db_error;
use glue::errors::{NanoServiceError, NanoServiceErrorStatus};

use crate::to_do_items::schema::ToDoItem;

#[cfg(feature = "sqlx-postgres")]
use super::super::descriptors::SqlxPostgresDescriptor;
pub trait GetAll {
    fn get_all(
        &self,
        user_id: &str,
    ) -> impl Future<Output = Result<Vec<ToDoItem>, NanoServiceError>> + Send;
}

pub trait GetById {
    fn get_by_id(
        &self,
        id: i32,
        user_id: &str,
    ) -> impl Future<Output = Result<ToDoItem, NanoServiceError>> + Send;
}

#[cfg(feature = "sqlx-postgres")]
impl GetAll for SqlxPostgresDescriptor {
    async fn get_all(&self, user_id: &str) -> Result<Vec<ToDoItem>, NanoServiceError> {
        let items = sqlx::query_as::<_, ToDoItem>(
            "SELECT t.id, t.title, t.status FROM to_do_items t \
             INNER JOIN user_connections uc ON uc.to_do_id = t.id \
             WHERE uc.user_id = $1 ORDER BY t.id",
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await
        .map_err(db_error)?;

        Ok(items)
    }
}

#[cfg(feature = "sqlx-postgres")]
impl GetById for SqlxPostgresDescriptor {
    async fn get_by_id(&self, id: i32, user_id: &str) -> Result<ToDoItem, NanoServiceError> {
        sqlx::query_as::<_, ToDoItem>(
            "SELECT t.id, t.title, t.status FROM to_do_items t \
             INNER JOIN user_connections uc ON uc.to_do_id = t.id \
             WHERE uc.user_id = $1 AND t.id = $2",
        )
        .bind(user_id)
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(db_error)?
        .ok_or_else(|| NanoServiceError::new("Item not found", NanoServiceErrorStatus::NotFound))
    }
}
