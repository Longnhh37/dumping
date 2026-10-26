use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct CountdownFuture {
    count: u32,
}

impl Future for CountdownFuture {
    type Output = &'static str;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.count == 0 {
            println!("Liftoff");
            Poll::Ready("Liftoff")
        } else {
            println!("{}...", self.count);
            self.count -= 1;
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
