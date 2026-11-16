use std::future::Future;

use crate::to_do_items::schema::ToDoItem;
use glue::errors::NanoServiceError;
use glue::db::db_error;

#[cfg(feature = "sqlx-postgres")]
use super::super::descriptors::SqlxPostgresDescriptor;
use glue::errors::NanoServiceErrorStatus;

pub trait GetAll {
    fn get_all(&self) -> impl Future<Output = Result<Vec<ToDoItem>, NanoServiceError>> + Send;
}

pub trait GetByName {
    fn get_by_name(
        &self,
        name: &str,
    ) -> impl Future<Output = Result<ToDoItem, NanoServiceError>> + Send;
}

#[cfg(feature = "sqlx-postgres")]
impl GetAll for SqlxPostgresDescriptor {
    async fn get_all(&self) -> Result<Vec<ToDoItem>, NanoServiceError> {
        let items =
            sqlx::query_as::<_, ToDoItem>("SELECT name, status FROM to_do_items ORDER BY name")
                .fetch_all(&self.pool)
                .await
                .map_err(db_error)?;

        Ok(items)
    }
}

#[cfg(feature = "sqlx-postgres")]
impl GetByName for SqlxPostgresDescriptor {
    async fn get_by_name(&self, name: &str) -> Result<ToDoItem, NanoServiceError> {
        sqlx::query_as::<_, ToDoItem>("SELECT name, status FROM to_do_items WHERE name = $1")
            .bind(name)
            .fetch_optional(&self.pool)
            .await
            .map_err(db_error)?
            .ok_or_else(|| {
                NanoServiceError::new("Item not found", NanoServiceErrorStatus::NotFound)
            })
    }
}
