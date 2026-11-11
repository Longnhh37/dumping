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
    crate::connections::sqlx_postgres::SQLX_POSTGRES_POOL, glue::errors::NanoServiceErrorStatus,
};

pub trait GetAll {
    fn get_all() -> impl Future<Output = Result<Vec<ToDoItem>, NanoServiceError>> + Send;
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
