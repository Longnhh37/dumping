use zero2prod::{
    config::get_config,
    startup::Application,
    telemetry::{get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    // set up log + trace
    let subscriber = get_subscriber("zero2prod".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let config = get_config().expect("failed to read config");

    let app = Application::build(config).await?;
    app.run_until_stopped().await?;

    Ok(())
}
