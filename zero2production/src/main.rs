use zero2prod::{
    config::get_config,
    startup::Application,
    telemetry::{LogFormat, get_subscriber, init_subscriber},
};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();

    let cfg = get_config().expect("failed to read config");

    let log_format = LogFormat::from(cfg.app.log_format.as_str());

    // set up log + trace
    let subscriber = get_subscriber(
        "zero2prod".into(),
        cfg.app.log_level.clone(),
        std::io::stdout,
        log_format,
    );
    init_subscriber(subscriber);

    let app = Application::build(cfg).await?;
    app.run_until_stopped().await?;

    Ok(())
}
