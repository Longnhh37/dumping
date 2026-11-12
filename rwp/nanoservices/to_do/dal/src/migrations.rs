use glue::errors::NanoServiceError;

use crate::connections::sqlx_postgres::SQLX_POSTGRES_POOL;

pub async fn run_migrations() -> Result<(), NanoServiceError> {
    println!("Migrating to-do database...");

    let mut migrations = sqlx::migrate!("./migrations");
    migrations.set_ignore_missing(true);

    migrations.run(&*SQLX_POSTGRES_POOL).await.map_err(|e| {
        NanoServiceError::new(e.to_string(), glue::errors::NanoServiceErrorStatus::Unknown)
    })?;

    println!("to-do database migrations completed");
    Ok(())
}
