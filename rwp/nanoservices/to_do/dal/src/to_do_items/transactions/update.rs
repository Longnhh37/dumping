use std::future::Future;

use crate::to_do_items::schema::ToDoItem;
use glue::errors::NanoServiceError;

#[cfg(feature = "json-file")]
use {
    super::super::descriptors::JsonFileDescriptor,
    crate::json_file::{get_all, save_all},
    std::collections::HashMap,
};

#[cfg(feature = "sqlx-postgres")]
use super::super::descriptors::SqlxPostgresDescriptor;

#[cfg(any(feature = "json-file", feature = "sqlx-postgres"))]
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
        RETURNING *",
        )
        .bind(item.name)
        .bind(item.status.to_string())
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| NanoServiceError::new(e.to_string(), NanoServiceErrorStatus::Unknown))?
        .ok_or_else(|| NanoServiceError::new("Item not found", NanoServiceErrorStatus::NotFound))
    }
}

#[cfg(feature = "json-file")]
impl UpdateOne for JsonFileDescriptor {
    async fn update_one(&self, item: ToDoItem) -> Result<ToDoItem, NanoServiceError> {
        let mut tasks = get_all::<ToDoItem>().unwrap_or_else(|_| HashMap::new());
        if !tasks.contains_key(&item.name) {
            return Err(NanoServiceError::new(
                format!("Item with name {} not found", item.name),
                NanoServiceErrorStatus::NotFound,
            ));
        }
        tasks.insert(item.name.clone(), item.clone());
        save_all(&tasks)?;

        Ok(item)
    }
}
