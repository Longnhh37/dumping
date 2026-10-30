use futures::stream::{self, StreamExt};

#[derive(Debug)]
struct Stats {
    count: usize,
    min: f64,
    max: f64,
    sum: f64,
}

async fn compute_stats<S: futures::Stream<Item = f64>>(stream: S) -> Stats {
    stream
        .fold(
            Stats {
                count: 0,
                min: f64::INFINITY,
                max: f64::NEG_INFINITY,
                sum: 0.0,
            },
            |mut acc, value| async move {
                acc.count += 1;
                acc.min = acc.min.min(value);
                acc.max = acc.max.max(value);
                acc.sum += value;
                acc
            },
        )
        .await
}

#[tokio::main]
async fn main() {
    let readings = stream::iter(vec![23.4, 25.4, 29.8, 15.5, 28.0]);
    let stats = compute_stats(readings).await;
    println!("{stats:?}");
}
