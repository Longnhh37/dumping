use tokio::net::TcpListener;
use tracing::Level;

use glue::{
    db::pool_from_env,
    log::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer, logger::init_logger},
};
use to_do_axum_server::api;
use to_do_dal::{migrations::run_migrations, to_do_items::descriptors::SqlxPostgresDescriptor};

#[cfg(feature = "auth-core-postgres")]
use auth_dal::users::descriptors::SqlxPostgresDescriptor as AuthPostgresDescriptor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    init_logger();
    let host = std::env::var("TO_DO_WEB_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("TO_DO_WEB_PORT").unwrap_or_else(|_| "8080".to_string());
    let address = format!("{host}:{port}");

    let pool = pool_from_env("TO_DO")?;
    run_migrations(&pool).await?;

    let listener = TcpListener::bind(&address).await?;
    let todo_store = SqlxPostgresDescriptor::new(pool);

    #[cfg(all(feature = "auth-http", not(feature = "auth-core-postgres")))]
    let app = api::basic_actions::basic_actions_router_http(todo_store);

    #[cfg(feature = "auth-core-postgres")]
    let app = {
        let auth_pool = pool_from_env("AUTH")?;
        api::basic_actions::basic_actions_router_core_postgres(
            todo_store,
            AuthPostgresDescriptor::new(auth_pool),
        )
    };

    tracing::info!(%address, "to-do server listening");
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
        .on_request(DefaultOnRequest::new().level(Level::INFO))
        .on_response(DefaultOnResponse::new().level(Level::INFO));
    axum::serve(listener, app.layer(trace_layer)).await?;
    Ok(())
}
