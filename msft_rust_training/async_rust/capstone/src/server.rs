use std::time::Duration;

use tokio::{net::TcpListener, sync::watch, time::timeout};
use tokio_util::task::TaskTracker;
use tracing::{debug, error, info, warn};

use crate::{
    client::{self, Context},
    error::ChatError,
    message::Message,
    room::Rooms,
};

#[derive(Debug, Clone, Copy)]
pub struct Config {
    pub idle_timeout: Duration,
    pub room_capacity: usize,
    pub drain_timeout: Duration,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            idle_timeout: Duration::from_secs(300),
            room_capacity: 100,
            drain_timeout: Duration::from_secs(10),
        }
    }
}

pub async fn run(
    listener: TcpListener,
    config: Config,
    shutdown: impl Future<Output = ()>,
) -> Result<(), ChatError> {
    info!(addr = %listener.local_addr()?, "chat server listening");

    let rooms = Rooms::new(config.room_capacity);
    let (shutdown_tx, shutdown_rx) = watch::channel(false);
    let tracker = TaskTracker::new();
    let mut next_id: u64 = 0;

    tokio::pin!(shutdown);

    loop {
        tokio::select! {
            accepted = listener.accept() => match accepted {
                Ok((socket, addr)) => {
                    next_id += 1;
                    let id = next_id;
                    let ctx = Context {
                        rooms: rooms.clone(),
                        config,
                        shutdown: shutdown_rx.clone(),
                    };
                    tracker.spawn(async move {
                        info!(id, %addr, "client connected");
                        match client::handle_connection(socket, id, ctx).await {
                            Ok(()) => info!(id, %addr, "client disconnected"),
                            Err(e) => debug!(id, %addr, error = %e, "session ended with error")
                        }
                    });
                }
                Err(e) => {
                    error!(error = %e, "accept failed");
                    tokio::time::sleep(Duration::from_millis(100)).await;
                }
            },

            () = &mut shutdown => {
                info!("shutdown signal received");
                break;
            }
        }
    }

    // graceful shuwdown

    // 1. Stop accepting new connections
    drop(listener);

    // 2. Tell every room, THEN flip the flag. Clients drain their queue
    rooms.broadcast_all(Message::server("Server shutting down...")).await;
    let _ = shutdown_tx.send(true);

    // 3. Wait for in-flight client tasks with a timeout
    tracker.close();
    if timeout(config.drain_timeout, tracker.wait()).await.is_err() {
        warn!(remaining = tracker.len(), "drain time out, exiting anyway");
    }

    info!("server stopped");
    Ok(())
}
