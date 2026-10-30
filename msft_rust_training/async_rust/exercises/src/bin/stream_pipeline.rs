use futures::stream::{self, StreamExt};
use tokio::time::{Duration, sleep};

#[tokio::main]
async fn main() {
    let res = stream::iter(1u64..=100)
        .filter(|&x| futures::future::ready(x & 1 == 0))
        .map(|x| x * x)
        .map(|x| async move {
            sleep(Duration::from_millis(50)).await;
            println!("Processed: {x}");
            x
        })
        .buffer_unordered(10)
        .collect::<Vec<_>>()
        .await;

    println!("Got {} results", res.len());
    println!("Sum: {}", res.iter().sum::<u64>());
}
