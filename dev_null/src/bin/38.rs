use std::collections::HashMap;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time::{Instant, interval, sleep};

const HEARTBEAT_INTERVAL: Duration = Duration::from_millis(500);
const TIMEOUT: Duration = Duration::from_secs(2);

async fn worker(id: u32, tx: mpsc::Sender<u32>, beats_before_crash: u32) {
    let mut ticker = interval(HEARTBEAT_INTERVAL);
    for _ in 0..beats_before_crash {
        ticker.tick().await;
        if tx.send(id).await.is_err() {
            return;
        }
    }
}

async fn monitor(mut rx: mpsc::Receiver<u32>, nodes: &[u32]) {
    let start = Instant::now();
    let mut last_seen: HashMap<u32, Instant> = nodes.iter().map(|&n| (n, start)).collect();
    let mut dead: Vec<u32> = Vec::new();
    let mut check = interval(Duration::from_millis(250));

    loop {
        tokio::select! {
            Some(id) = rx.recv() => {
                last_seen.insert(id, Instant::now());
                println!("heartbeat from node {id}");
            }
            _ = check.tick() => {
                for (&id, &seen) in &last_seen {
                    if seen.elapsed() > TIMEOUT && !dead.contains(&id) {
                        println!("node {id} suspected DEAD (no heartbeat for {:?})", seen.elapsed());
                        dead.push(id);
                    }
                }
                if dead.len() == nodes.len() {
                    return;
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel(32);

    tokio::spawn(worker(1, tx.clone(), 3));
    tokio::spawn(worker(2, tx.clone(), 8));
    drop(tx);

    monitor(rx, &[1, 2]).await;
    sleep(Duration::from_millis(100)).await;
}
