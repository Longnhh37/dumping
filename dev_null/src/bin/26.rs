use crossbeam_channel::bounded;
use std::thread;

fn main() {
    // Bounded queue gives natural backpressure to producers
    let (tx, rx) = bounded::<u32>(64);

    let handles: Vec<_> = (0..4)
        .map(|id| {
            let rx = rx.clone(); // MPMC: all workers share one queue
            thread::spawn(move || {
                // An idle worker blocks here and takes the next task
                for task in rx {
                    println!("worker {id} handles task {task}");
                }
            })
        })
        .collect();

    for task in 0..16u32 {
        tx.send(task).unwrap();
    }

    drop(tx);
    for h in handles {
        h.join().unwrap();
    }
}
