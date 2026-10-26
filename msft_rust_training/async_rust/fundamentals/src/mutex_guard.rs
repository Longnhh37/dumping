use std::sync::Arc;

// BAD: guard is alive across .await
async fn bad(m: &std::sync::Mutex<Vec<u32>>) {
    let mut g = m.lock().unwrap();
    g.push(1);
    some_io().await; // guard is still held here
    g.push(2);
}

// GOOD: scope the guard so it drops before .await
async fn good(m: &std::sync::Mutex<Vec<u32>>) {
    {
        let mut g = m.lock().unwrap();
        g.push(1);
    } // guard dropped here
    some_io().await;
    m.lock().unwrap().push(2);
}

async fn ok(m: Arc<tokio::sync::Mutex<Vec<u32>>>) {
    let mut g = m.lock().await; // suspends the task, does not block the thread
    g.push(1);
    some_io().await; // OK: guard is Send, lock stays held
    g.push(2);
}

async fn some_io() {}
