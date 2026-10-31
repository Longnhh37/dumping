use tracing::{debug, error, info, trace, warn};
use tracing_subscriber::EnvFilter;

fn main() {
    // Read the filter from RUST_LOG
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_target(true)
        .with_thread_ids(false)
        .init();

    trace!("very verbose");
    debug!("debug info");
    info!("server started");
    warn!("disk usage high");
    error!("something failed");
}

// usage
//      RUST_LOG=debug cargo run
//      RUST_LOG=my_app=debug,hyper=warn cargo run   # per-crate level
//      RUST_LOG="my_app::db=trace,info" cargo run   # per-module level
