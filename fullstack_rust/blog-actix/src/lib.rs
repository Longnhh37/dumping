pub mod app;
pub mod errors;

mod db;
mod dto;
mod models;
mod routes;
mod schema;
mod services;

use diesel::{
    SqliteConnection,
    r2d2::{self, ConnectionManager},
};

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;


