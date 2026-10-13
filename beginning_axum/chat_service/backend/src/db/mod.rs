use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};

/// Connect to the configured database and apply pending migrations.
pub async fn init(database_url: &str) -> DatabaseConnection {
    let connection = Database::connect(database_url)
        .await
        .expect("Failed to connect to database");

    Migrator::up(&connection, None)
        .await
        .expect("Failed to apply database migrations");

    connection
}
