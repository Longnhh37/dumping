use std::future::Future;

use crate::to_do_items::schema::ToDoItem;
use glue::errors::{NanoServiceError, NanoServiceErrorStatus};

#[cfg(feature = "json-file")]
use {
    super::super::descriptors::JsonFileDescriptor,
    crate::json_file::{get_all, save_all},
    std::collections::HashMap,
};

#[cfg(feature = "sqlx-postgres")]
use {
    super::super::descriptors::SqlxPostgresDescriptor,
    crate::connections::sqlx_postgres::SQLX_POSTGRES_POOL,
};

pub trait DeleteOne {
    fn delete_one(name: String) -> impl Future<Output = Result<ToDoItem, NanoServiceError>> + Send;
}

#[cfg(feature = "sqlx-postgres")]
impl DeleteOne for SqlxPostgresDescriptor {
    fn delete_one(name: String) -> impl Future<Output = Result<ToDoItem, NanoServiceError>> + Send {
        sqlx_postgres_delete_one(name)
    }
}

#[cfg(feature = "sqlx-postgres")]
async fn sqlx_postgres_delete_one(name: String) -> Result<ToDoItem, NanoServiceError> {
    sqlx::query_as::<_, ToDoItem>(
        "
        DELETE FROM to_do_items
        WHERE name = $1
        RETURNING *",
    )
    .bind(name)
    .fetch_optional(&*SQLX_POSTGRES_POOL)
    .await
    .map_err(|e| NanoServiceError::new(e.to_string(), NanoServiceErrorStatus::Unknown))?
    .ok_or_else(|| NanoServiceError::new("Item not found", NanoServiceErrorStatus::NotFound))
}

#[cfg(feature = "json-file")]
impl DeleteOne for JsonFileDescriptor {
    fn delete_one(name: String) -> impl Future<Output = Result<ToDoItem, NanoServiceError>> + Send {
        json_file_delete_one(name)
    }
}

#[cfg(feature = "json-file")]
async fn json_file_delete_one(name: String) -> Result<ToDoItem, NanoServiceError> {
    let mut tasks = get_all::<ToDoItem>().unwrap_or_else(|_| HashMap::new());
    let to_do_item = tasks.remove(&name).ok_or_else(|| {
        NanoServiceError::new(
            "Item not found".to_string(),
            NanoServiceErrorStatus::NotFound,
        )
    })?;
    save_all(&tasks)?;

    Ok(to_do_item)
}
