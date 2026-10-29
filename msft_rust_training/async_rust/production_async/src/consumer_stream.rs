use futures::stream::{self, StreamExt};
use std::time::Duration;

async fn stream_examples() {
    let s = stream::iter(vec![1, 2, 3, 4, 5]);

    // for_each — process each item
    s.for_each(|x| async move {
        println!("{x}");
    })
    .await;

    // map + collect
    let doubled: Vec<i32> = stream::iter(vec![1, 2, 3]).map(|x| x * 2).collect().await;

    // filter
    let evens: Vec<i32> = stream::iter(1..=10)
        .filter(|x| futures::future::ready(x % 2 == 0))
        .collect()
        .await;

    // buffer_unordered — process N items concurrently
    let results: Vec<_> = stream::iter(vec!["url1", "url2", "url3"])
        .map(|url| async move {
            // Simulate HTTP fetch
            tokio::time::sleep(Duration::from_millis(100)).await;
            format!("response from {url}")
        })
        .buffer_unordered(10) // Up to 10 concurrent fetches
        .collect()
        .await;

    // take, skip, zip, chain — just like Iterator
    let first_three: Vec<i32> = stream::iter(1..=100).take(3).collect().await;
}
