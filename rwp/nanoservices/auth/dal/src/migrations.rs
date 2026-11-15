use glue::errors::NanoServiceError;

use sqlx::PgPool;

pub async fn run_migrations(pool: &PgPool) -> Result<(), NanoServiceError> {
    println!("migrating auth database...");
    let mut migrations = sqlx::migrate!("./migrations");
    migrations.set_ignore_missing(true);
    migrations.run(pool).await.map_err(|e| {
        NanoServiceError::new(e.to_string(), glue::errors::NanoServiceErrorStatus::Unknown)
    })?;
    println!("auth database migrations completed");
    Ok(())
}
