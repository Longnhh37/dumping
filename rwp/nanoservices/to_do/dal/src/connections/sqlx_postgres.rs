use std::env;
use std::sync::LazyLock;

use sqlx::postgres::{PgPool, PgPoolOptions};

pub static SQLX_POSTGRES_POOL: LazyLock<PgPool> = LazyLock::new(|| {
    let connection_string = env::var("TO_DO_DB_URL").unwrap();
    let max_connections = env::var("TO_DO_MAX_CONNECTIONS")
        .unwrap_or_else(|_| "5".to_string())
        .trim()
        .parse::<u32>()
        .map_err(|_| "could not parse max connections".to_string())
        .unwrap();
    let pool = PgPoolOptions::new().max_connections(max_connections);
    pool.connect_lazy(&connection_string)
        .expect("Failed to create DB pool")
});
