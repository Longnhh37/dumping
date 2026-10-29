// Build a task processor with a channel-based work queue,
//      N worker tasks, and graceful shutdown on Ctrl+C.
// Workers should finish in-flight work before exiting.

use tokio::sync::{mpsc, watch};
use tokio::time::{Duration, sleep};

struct WorkItem {
    id: u64,
    payload: String,
}

#[tokio::main]
async fn main() {
    let (work_tx, work_rx) = mpsc::channel::<WorkItem>(100);
    let (shutdown_tx, shutdown_rx) = watch::channel(false);
    let work_rx = std::sync::Arc::new(tokio::sync::Mutex::new(work_rx));

    let mut handles = Vec::new();
    for id in 0..4 {
        let rx = work_rx.clone();
        let mut shutdown = shutdown_rx.clone();
        handles.push(tokio::spawn(async move {
            loop {
                let item = {
                    let mut rx = rx.lock().await;
                    tokio::select! {
                        item = rx.recv() => item,
                        _ = shutdown.changed() => {
                            if *shutdown.borrow() { None } else { continue }
                        }
                    }
                };
                match item {
                    Some(work) => {
                        println!("Worker {id}: processing {}", work.id);
                        sleep(Duration::from_millis(200)).await;
                    }
                    None => break,
                }
            }
        }));
    }

    for i in 0..20 {
        let _ = work_tx
            .send(WorkItem {
                id: i,
                payload: format!("task-{i}"),
            })
            .await;
        sleep(Duration::from_millis(50)).await;
    }

    tokio::signal::ctrl_c().await.unwrap();
    shutdown_tx.send(true).unwrap();
    for h in handles {
        let _ = h.await;
    }
    println!("Shutdown cleanly");
}
