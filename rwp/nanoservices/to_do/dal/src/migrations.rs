use crate::connections::sqlx_postgres::SQLX_POSTGRES_POOL;

pub async fn run_migrations() {
    println!("Migrating to-do database...");
    let mut migrations = sqlx::migrate!("./migrations");
    migrations.set_ignore_missing(true);
    migrations.run(&*SQLX_POSTGRES_POOL).await.unwrap();
    println!("to-do database migrations completed");
}
