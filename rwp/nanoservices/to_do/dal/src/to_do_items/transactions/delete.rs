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
    fn delete_one(title: String)
    -> impl Future<Output = Result<ToDoItem, NanoServiceError>> + Send;
}
