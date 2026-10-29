use std::sync::Arc;
use std::{future::Future, time::Duration};
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

    // semaphore
    let urls = vec!["sample.com", "example.com"];
    let fetch = async |url: &str| -> String {
        std::thread::sleep(Duration::from_secs(3));
        "Hello".to_string()
    };
    let tasks: Vec<_> = urls
        .into_iter()
        .map(|url| move || async move { fetch(url).await })
        .collect();
    let res = run_with_limit(tasks, 10).await;
}

async fn run_with_limit<F, Fut, T>(tasks: Vec<F>, limit: usize) -> Vec<T>
where
    F: FnOnce() -> Fut + Send + 'static,
    Fut: Future<Output = T> + Send + 'static,
    T: Send + 'static,
{
    let semaphore = Arc::new(Semaphore::new(limit));
    let mut handles = Vec::new();

    for task in tasks {
        let permit = Arc::clone(&semaphore);
        let handle = tokio::spawn(async move {
            let _permit = permit.acquire().await.unwrap();
            task().await
        });
        handles.push(handle);
    }

    let mut res = Vec::new();
    for handle in handles {
        res.push(handle.await.unwrap());
    }
    res
}
