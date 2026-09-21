use std::{sync::atomic::AtomicU64, time::Instant};
use std::thread;

use rust_atomic_and_locks::{
    cache::{Padded, run},
    channel::{Channel2, Receiver, Sender},
    locks::{Mutex, MutextGuard},
    spinlock::SpinLock,
};

fn main() {
    // ================================================================
    // Spinlock
    // ================================================================
    let x = SpinLock::new(Vec::new());
    thread::scope(|s| {
        s.spawn(|| x.lock().push(1));
        s.spawn(|| {
            let mut g = x.lock();
            g.push(2);
            g.push(3);
        });
    });

    let g = x.lock();
    assert!(g.as_slice() == [1, 2, 3] || g.as_slice() == [2, 3, 1]);

    // ================================================================
    // Channel
    // ================================================================
    let mut channel = Channel2::new();

    thread::scope(|s| {
        let (sender, receiver) = channel.split();
        s.spawn(move || {
            sender.send("Hello world");
        });
        assert_eq!(receiver.receive(), "Hello world");
    });

    // ================================================================
    // Cache coherence and False sharing
    // ================================================================
    let same_line = [AtomicU64::new(0), AtomicU64::new(0)];
    let separate = [Padded(AtomicU64::new(0)), Padded(AtomicU64::new(0))];

    println!("same cache line:  {:?}", run(&same_line, |c| c));
    println!("different cache line:  {:?}", run(&separate, |p| &p.0));

    // ================================================================
    // Locks
    // ================================================================
    let m = Mutex::new(0);
    std::hint::black_box(&m);
    let start = Instant::now();
    for _ in 0..5_000_000 {
        *m.lock() += 1;
    }
    let duration = start.elapsed();
    println!("locked {} times in {:?}", *m.lock(), duration);

    println!("exit 0");
}
