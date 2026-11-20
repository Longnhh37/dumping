use tokio::sync::{mpsc, oneshot};

enum Msg {
    Incr(i64),
    Get(oneshot::Sender<i64>),
}

async fn counter_actor(mut rx: mpsc::Receiver<Msg>) {
    let mut count = 0_i64;
    while let Some(msg) = rx.recv().await {
        match msg {
            Msg::Incr(n) => count += n,
            Msg::Get(reply) => {
                let _ = reply.send(count);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let (tx, rx) = mpsc::channel(32);
    tokio::spawn(counter_actor(rx));

    tx.send(Msg::Incr(5)).await.unwrap();
    tx.send(Msg::Incr(3)).await.unwrap();

    let (reply_tx, reply_rx) = oneshot::channel();
    tx.send(Msg::Get(reply_tx)).await.unwrap();
    println!("count = {}", reply_rx.await.unwrap());
}
