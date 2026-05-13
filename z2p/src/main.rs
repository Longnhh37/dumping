use std::net::TcpListener;

use secrecy::ExposeSecret;
use sqlx::PgPool;

use zero2prod::{
    config::get_config,
    startup::run,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config = get_config().expect("failed to read config");

    let pool = PgPool::connect(config.database.connection_string().expose_secret())
        .await
        .expect("failed to connect to postgres");

    let address = format!("{}:{}", config.database.host, config.app_port);
    let listener = TcpListener::bind(address)?;

    run(listener, pool)?.await
}
