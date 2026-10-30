use futures::stream::{self, StreamExt};
use tokio::time::{Duration, sleep};

async fn fetch_urls(urls: Vec<String>) -> Vec<Result<String, String>> {
    stream::iter(urls)
        .map(|url| async move {
            println!("Fetching: {url}");

            match reqwest::get(&url).await {
                Ok(resp) => match resp.text().await {
                    Ok(body) => Ok(body),
                    Err(e) => Err(format!("{url}: {e}")),
                },
                Err(e) => Err(format!("{url}: {e}")),
            }
        })
        .buffer_unordered(5)
        .collect::<Vec<_>>()
        .await
}

fn main() {}
