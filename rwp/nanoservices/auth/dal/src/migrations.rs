use glue::errors::NanoServiceError;

use crate::connections::sqlx_postgres::SQLX_POSTGRES_POOL;

pub async fn run_migrations() -> Result<(), NanoServiceError> {
    println!("migrating auth database...");
    let mut migrations = sqlx::migrate!("./migrations");
    migrations.set_ignore_missing(true);
    migrations.run(&*SQLX_POSTGRES_POOL).await.unwrap();
    println!("auth database migrations completed");
    Ok(())
}
