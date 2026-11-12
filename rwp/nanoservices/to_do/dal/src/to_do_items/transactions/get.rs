use std::future::Future;

use crate::to_do_items::schema::ToDoItem;
use glue::errors::NanoServiceError;

#[cfg(feature = "json-file")]
use {
    super::super::descriptors::JsonFileDescriptor, crate::json_file::get_all,
    std::collections::HashMap,
};

#[cfg(feature = "sqlx-postgres")]
use {
    super::super::descriptors::SqlxPostgresDescriptor,
    crate::connections::sqlx_postgres::SQLX_POSTGRES_POOL,
};

#[cfg(any(feature = "json-file", feature = "sqlx-postgres"))]
use glue::errors::NanoServiceErrorStatus;

pub trait GetAll {
    fn get_all() -> impl Future<Output = Result<Vec<ToDoItem>, NanoServiceError>> + Send;
}

pub trait GetByName {
    fn get_by_name(name: String)
    -> impl Future<Output = Result<ToDoItem, NanoServiceError>> + Send;
}

#[cfg(feature = "sqlx-postgres")]
impl GetAll for SqlxPostgresDescriptor {
    fn get_all() -> impl Future<Output = Result<Vec<ToDoItem>, NanoServiceError>> + Send {
        sqlx_postgres_get_all()
    }
}

#[cfg(feature = "sqlx-postgres")]
async fn sqlx_postgres_get_all() -> Result<Vec<ToDoItem>, NanoServiceError> {
    let items = sqlx::query_as::<_, ToDoItem>("SELECT * FROM to_do_items")
        .fetch_all(&*SQLX_POSTGRES_POOL)
        .await
        .map_err(|e| NanoServiceError::new(e.to_string(), NanoServiceErrorStatus::Unknown))?;

    Ok(items)
}

#[cfg(feature = "json-file")]
impl GetAll for JsonFileDescriptor {
    fn get_all() -> impl Future<Output = Result<Vec<ToDoItem>, NanoServiceError>> + Send {
        json_file_get_all()
    }
}

#[cfg(feature = "json-file")]
async fn json_file_get_all() -> Result<Vec<ToDoItem>, NanoServiceError> {
    let tasks = get_all::<ToDoItem>().unwrap_or_else(|_| HashMap::new());
    let items = tasks.values().cloned().collect();

    Ok(items)
}

#[cfg(feature = "sqlx-postgres")]
impl GetByName for SqlxPostgresDescriptor {
    fn get_by_name(
        name: String,
    ) -> impl Future<Output = Result<ToDoItem, NanoServiceError>> + Send {
        sqlx_postgres_get_by_name(name)
    }
}

#[cfg(feature = "sqlx-postgres")]
async fn sqlx_postgres_get_by_name(name: String) -> Result<ToDoItem, NanoServiceError> {
    sqlx::query_as::<_, ToDoItem>("SELECT * FROM to_do_items WHERE name = $1")
        .bind(name)
        .fetch_optional(&*SQLX_POSTGRES_POOL)
        .await
        .map_err(|e| NanoServiceError::new(e.to_string(), NanoServiceErrorStatus::Unknown))?
        .ok_or_else(|| NanoServiceError::new("Item not found", NanoServiceErrorStatus::NotFound))
}

#[cfg(feature = "json-file")]
impl GetByName for JsonFileDescriptor {
    fn get_by_name(
        name: String,
    ) -> impl Future<Output = Result<ToDoItem, NanoServiceError>> + Send {
        json_file_get_by_name(name)
    }
}

#[cfg(feature = "json-file")]
async fn json_file_get_by_name(name: String) -> Result<ToDoItem, NanoServiceError> {
    let tasks = get_all::<ToDoItem>()?;
    tasks
        .get(&name)
        .cloned()
        .ok_or_else(|| NanoServiceError::new("Item not found", NanoServiceErrorStatus::NotFound))
}
