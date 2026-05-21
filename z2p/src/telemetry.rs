use tokio::task::JoinHandle;
use tracing::{Subscriber, subscriber::set_global_default};
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{EnvFilter, Registry, fmt, fmt::MakeWriter, layer::SubscriberExt};

pub enum LogFormat {
    Pretty,
    Json,
}

impl From<&str> for LogFormat {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "pretty" => LogFormat::Pretty,
            _ => LogFormat::Json,
        }
    }
}

pub fn get_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
    log_format: LogFormat,
) -> Box<dyn Subscriber + Send + Sync>
where
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));

    match log_format {
        LogFormat::Pretty => {
            let subscriber = Registry::default().with(env_filter).with(
                fmt::layer()
                    .with_writer(sink)
                    .pretty()
                    .with_target(true)
                    .with_line_number(true),
            );
            Box::new(subscriber)
        }
        LogFormat::Json => {
            let subscriber = Registry::default()
                .with(env_filter)
                .with(JsonStorageLayer)
                .with(BunyanFormattingLayer::new(name, sink));
            Box::new(subscriber)
        }
    }
}

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    LogTracer::init().expect("failed to set logger");

    set_global_default(subscriber).expect("failed to set subscriber");
}

pub fn spawn_blocking_with_tracing<F, R>(f: F) -> JoinHandle<R>
where
    F: FnOnce() -> R + Send + 'static,
    R: Send + 'static,
{
    let current_span = tracing::Span::current();
    tokio::task::spawn_blocking(move || current_span.in_scope(f))
}
