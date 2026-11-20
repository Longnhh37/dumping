use tokio::net::TcpListener;
use tracing::Level;

use auth_axum_server::api;
use auth_dal::{migrations::run_migrations, users::descriptors::SqlxPostgresDescriptor};
use auth_kernel::user_session::descriptors::RedisSessionDescriptor;
use glue::{
    db::pool_from_env,
    log::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer, logger::init_logger},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    init_logger();
    let host = std::env::var("AUTH_WEB_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = std::env::var("AUTH_WEB_PORT").unwrap_or_else(|_| "8081".to_string());
    let address = format!("{host}:{port}");

    let pool = pool_from_env("AUTH")?;
    run_migrations(&pool).await?;

    let listener = TcpListener::bind(&address).await?;
    let store = SqlxPostgresDescriptor::new(pool);
    let app = api::router(api::AppState::new(
        store.clone(),
        RedisSessionDescriptor::new(store),
    ))
    .layer(
        TraceLayer::new_for_http()
            .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
            .on_request(DefaultOnRequest::new().level(Level::INFO))
            .on_response(DefaultOnResponse::new().level(Level::INFO)),
    );

    tracing::info!(%address, "auth server listening");
    axum::serve(listener, app).await?;
    Ok(())
}
