use std::sync::Arc;
use tokio::sync::{Mutex, RwLock, Semaphore, broadcast, mpsc, oneshot, watch};

#[tokio::main]
async fn main() {
    // --- Mutex ---
    // Async mutex: the lock() method is async and won't block the thread
    let data = Arc::new(Mutex::new(vec![1, 2, 3]));
    {
        let mut guard = data.lock().await; // Non-blocking lock
        guard.push(4);
    } // Guard dropped here — lock released

    // --- Channels ---
    // mpsc: Multiple producer, single consumer
    let (tx, mut rx) = mpsc::channel::<String>(100); // Bounded buffer

    tokio::spawn(async move {
        tx.send("hello".into()).await.unwrap();
    });

    let msg = rx.recv().await.unwrap();

    // oneshot: Single value, single consumer
    let (tx, rx) = oneshot::channel::<i32>();
    tx.send(42).unwrap(); // No await needed — either sends or fails
    let val = rx.await.unwrap();

    // broadcast: Multiple producers, multiple consumers (all get every message)
    let (tx, _) = broadcast::channel::<String>(100);
    let mut rx1 = tx.subscribe();
    let mut rx2 = tx.subscribe();

    // watch: Single value, multiple consumers (only latest value)
    let (tx, rx) = watch::channel(0u64);
    tx.send(42).unwrap();
    println!("Latest: {}", *rx.borrow());
}
