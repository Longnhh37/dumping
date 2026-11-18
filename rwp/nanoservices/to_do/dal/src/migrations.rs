use glue::errors::{NanoServiceError, NanoServiceErrorStatus, ResultExt};

use sqlx::PgPool;

pub async fn run_migrations(pool: &PgPool) -> Result<(), NanoServiceError> {
    println!("Migrating to-do database...");

    let mut migrations = sqlx::migrate!("./migrations");
    migrations.set_ignore_missing(true);
    migrations
        .run(pool)
        .await
        .eject(NanoServiceErrorStatus::Unknown)?;

    println!("to-do database migrations completed");
    Ok(())
}
