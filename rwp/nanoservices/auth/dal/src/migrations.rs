use glue::errors::{NanoServiceError, NanoServiceErrorStatus, ResultExt};

use sqlx::PgPool;

pub async fn run_migrations(pool: &PgPool) -> Result<(), NanoServiceError> {
    println!("migrating auth database...");

    let mut migrations = sqlx::migrate!("./migrations");
    migrations.set_ignore_missing(true);
    migrations
        .run(pool)
        .await
        .eject(NanoServiceErrorStatus::Unknown)?;

    println!("auth database migrations completed");
    Ok(())
}
