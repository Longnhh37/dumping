use std::sync::atomic::{AtomicU64, Ordering::Relaxed};
use std::thread;
use std::time::{Duration, Instant};

#[repr(align(64))]
pub struct Padded(pub AtomicU64);

pub fn run<T: Sync>(slots: &[T; 2], get: impl Fn(&T) -> &AtomicU64 + Sync) -> Duration {
    let get = &get;
    let start = Instant::now();
    thread::scope(|s| {
        for slot in slots {
            s.spawn(move || {
                let counter = get(slot);
                for _ in 0..50_000_000 {
                    counter.fetch_add(1, Relaxed);
                }
            });
        }
    });
    start.elapsed()
}
