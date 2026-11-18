use tokio::net::TcpListener;

use glue::db::pool_from_env;
use to_do_axum_server::api;
use to_do_dal::{migrations::run_migrations, to_do_items::descriptors::SqlxPostgresDescriptor};

#[cfg(feature = "auth-core-postgres")]
use auth_dal::users::descriptors::SqlxPostgresDescriptor as AuthPostgresDescriptor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let host = std::env::var("TO_DO_WEB_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("TO_DO_WEB_PORT").unwrap_or_else(|_| "8080".to_string());
    let address = format!("{host}:{port}");

    let pool = pool_from_env("TO_DO")?;
    run_migrations(&pool).await?;

    let listener = TcpListener::bind(&address).await?;
    let todo_store = SqlxPostgresDescriptor::new(pool);

    #[cfg(feature = "auth-http")]
    let app = api::basic_actions::basic_actions_router(todo_store);

    #[cfg(feature = "auth-core-postgres")]
    let app = {
        let auth_pool = pool_from_env("AUTH")?;
        api::basic_actions::basic_actions_router(todo_store, AuthPostgresDescriptor::new(auth_pool))
    };

    println!("Serving app on {address}");
    axum::serve(listener, app).await?;
    Ok(())
}
