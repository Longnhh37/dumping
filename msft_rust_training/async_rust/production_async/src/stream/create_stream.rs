use futures::stream::{self, StreamExt};
use tokio::time::{Duration, interval};
use tokio_stream::wrappers::IntervalStream;

fn create_stream() {
    // 1. from an iter
    let s = stream::iter(vec![1, 2, 3]);

    // 2. from async generator
    use async_stream::stream;

    fn count_down(from: u32) -> impl futures::Stream<Item = u32> {
        stream! {
            for i in (0..=from).rev() {
                tokio::time::sleep(Duration::from_millis(500)).await;
                yield i;
            }
        }
    }

    // 3. from tokio interval
    let tick_stream = IntervalStream::new(interval(Duration::from_secs(1)));

    // 4. from a channel receiver (tokio_stream wrappers)
    let (tx, rx) = tokio::sync::mpsc::channel::<String>(100);
    let rx_stream = tokio_stream::wrappers::ReceiverStream::new(rx);

    // 5. from unfold (generate from async state)
    let s = stream::unfold(0u32, |state| async move {
        if state >= 5 {
            None
        } else {
            let next = state + 1;
            Some((state, next))
        }
    });
}
