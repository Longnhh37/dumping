use tracing_appender::rolling;
use tracing_subscriber::{EnvFilter, Layer, fmt, layer::SubscriberExt, util::SubscriberInitExt};

fn init_tracing() -> tracing_appender::non_blocking::WorkerGuard {
    // daily rotating file, written on a bg thread
    let file_appender = rolling::daily("logs", "app.log");
    let (file_writer, guard) = tracing_appender::non_blocking(file_appender);

    // Layer for terminal
    let stdout_layer = fmt::layer().pretty().with_filter(EnvFilter::new("debug"));

    // JSON layer for files
    let json_layer = fmt::layer()
        .json()
        .with_writer(file_writer)
        .with_current_span(true)
        .with_filter(EnvFilter::new("info"));

    tracing_subscriber::registry()
        .with(stdout_layer)
        .with(json_layer)
        .init();

    // must be kept alive until the program exists, or buffered logs are lost
    guard
}

fn main() {
    let _guard = init_tracing();
    tracing::info!("hello from both layers");
}
