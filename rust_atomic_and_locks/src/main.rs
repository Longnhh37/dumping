use std::thread;

use rust_atomic_and_locks::spinlock::SpinLock;

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
    println!("exit 0");
}
