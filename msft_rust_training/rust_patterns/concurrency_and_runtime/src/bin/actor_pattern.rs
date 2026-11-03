use tokio::sync::{mpsc, oneshot};
use tokio::task::JoinSet;

enum Msg {
    Increment,
    Decrement,
    Get(oneshot::Sender<i64>),
}

struct CounterActor {
    count: i64,
    rx: mpsc::Receiver<Msg>,
}

impl CounterActor {
    async fn run(mut self) {
        while let Some(msg) = self.rx.recv().await {
            match msg {
                Msg::Increment => self.count += 1,
                Msg::Decrement => self.count -= 1,
                Msg::Get(reply) => {
                    let _ = reply.send(self.count);
                }
            }
        }
    }
}

#[derive(Clone)]
struct Counter {
    tx: mpsc::Sender<Msg>,
}

impl Counter {
    fn spawn() -> Self {
        let (tx, rx) = mpsc::channel(64);
        tokio::spawn(CounterActor { count: 0, rx }.run());
        Self { tx }
    }

    async fn increment(&self) {
        let _ = self.tx.send(Msg::Increment).await;
    }

    async fn decrement(&self) {
        let _ = self.tx.send(Msg::Decrement).await;
    }

    async fn get(&self) -> Option<i64> {
        let (reply_tx, reply_rx) = oneshot::channel();
        self.tx.send(Msg::Get(reply_tx)).await.ok()?;
        reply_rx.await.ok()
    }
}

#[tokio::main]
async fn main() {
    let counter = Counter::spawn();

    let mut tasks = JoinSet::new();
    for _ in 0..10 {
        let counter = counter.clone();
        tasks.spawn(async move {
            for _ in 0..1000 {
                counter.increment().await;
            }
        });
    }
    while tasks.join_next().await.is_some() {}

    println!("Final count: {:?}", counter.get().await);
}
