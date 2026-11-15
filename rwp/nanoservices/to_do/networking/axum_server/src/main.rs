use tokio::net::TcpListener;

use to_do_axum_server::api;
use to_do_dal::{
    connections::sqlx_postgres::create_pool, migrations::run_migrations,
    to_do_items::descriptors::SqlxPostgresDescriptor,
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let host = std::env::var("TO_DO_WEB_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("TO_DO_WEB_PORT").unwrap_or_else(|_| "8080".to_string());
    let address = format!("{host}:{port}");

    let database_url = std::env::var("TO_DO_DB_URL").expect("TO_DO_DB_URL must be set");
    let max_connections = std::env::var("TO_DO_MAX_CONNECTIONS")
        .unwrap_or_else(|_| "5".to_string())
        .parse::<u32>()
        .expect("TO_DO_MAX_CONNECTIONS must be a positive integer");
    let pool = create_pool(&database_url, max_connections).expect("failed to create to-do pool");
    run_migrations(&pool)
        .await
        .expect("failed to run migrations");

    let listener = TcpListener::bind(&address).await?;
    let app = api::router(SqlxPostgresDescriptor::new(pool));

    println!("Serving app on {address}");
    axum::serve(listener, app).await
}
