use tokio::net::TcpListener;

use glue::db::pool_from_env;
use to_do_axum_server::api;
use to_do_dal::{migrations::run_migrations, to_do_items::descriptors::SqlxPostgresDescriptor};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let host = std::env::var("TO_DO_WEB_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("TO_DO_WEB_PORT").unwrap_or_else(|_| "8080".to_string());
    let address = format!("{host}:{port}");

    let pool = pool_from_env("TO_DO")?;
    run_migrations(&pool).await?;

    let listener = TcpListener::bind(&address).await?;
    let app = api::basic_actions::basic_actions_router(SqlxPostgresDescriptor::new(pool));

    println!("Serving app on {address}");
    axum::serve(listener, app).await?;
    Ok(())
}
