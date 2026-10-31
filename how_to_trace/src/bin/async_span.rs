// don't span.enter() across 'await'.
use tracing::{Instrument, info, info_span, instrument};

#[instrument]
async fn handle_request(id: u32) {
    info!("handling");
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;
    info!("done");
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_env_filter("info").init();

    handle_request(1).await;

    // attach a span to a future
    let span = info_span!("background job", job = "cleanup");
    tokio::spawn(
        async {
            info!("running in spawned task");
        }
        .instrument(span),
    )
    .await
    .unwrap();
}
