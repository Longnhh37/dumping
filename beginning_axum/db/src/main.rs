mod entities;

use sea_orm::{Database, DatabaseConnection, DbErr};

#[tokio::main]
async fn main() -> Result<(), DbErr> {
    dotenvy::dotenv().ok();
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db: DatabaseConnection = Database::connect(&url).await?;
    db.ping().await?;
    println!("Connected to postgreSQL");

    Ok(())
}
