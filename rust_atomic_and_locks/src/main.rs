use std::thread;

use rust_atomic_and_locks::{
    channels::{Channel2, Receiver, Sender},
    spinlock::SpinLock,
};

fn main() {
    // ================================================================
    // Spin lock
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

    println!("exit 0");
}
