use std::time::Duration;

#[derive(Debug)]
enum FetchError {
    Transient(String),
    Permanent(String),
}

impl FetchError {
    fn is_transient(&self) -> bool {
        matches!(self, Self::Transient(_))
    }
}

async fn retry<T, F, Fut>(max_attempts: u32, mut op: F) -> Result<T, FetchError>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, FetchError>>,
{
    let mut delay = Duration::from_millis(100);

    for attempt in 1..=max_attempts {
        match op().await {
            Ok(v) => return Ok(v),
            Err(e) if e.is_transient() && attempt < max_attempts => {
                tokio::time::sleep(delay).await;
                delay = (delay * 2).min(Duration::from_secs(5));
            }
            Err(e) => return Err(e),
        }
    }
    unreachable!("loop always return on last attempt");
}

fn main() {}
