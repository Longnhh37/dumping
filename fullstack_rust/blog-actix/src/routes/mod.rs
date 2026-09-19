use actix_web::web;
use diesel::{
    SqliteConnection,
    r2d2::{ConnectionManager, PooledConnection},
};

use crate::{DbPool, errors::AppError};

pub mod comments;
pub mod posts;
pub mod users;

/// checked-out connection from the pool
pub type DbConn = PooledConnection<ConnectionManager<SqliteConnection>>;

fn get_conn(pool: &DbPool) -> Result<DbConn, AppError> {
    pool.get()
        .map_err(|e| AppError::DatabasePool(e.to_string()))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    users::configure(cfg);
    posts::configure(cfg);
    comments::configure(cfg);
}
