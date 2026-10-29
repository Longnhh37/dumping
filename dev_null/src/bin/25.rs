use std::sync::mpsc;
use std::thread;

fn main() {
    let mut senders = Vec::new();
    let mut handles = Vec::new();

    for id in 0..4 {
        // One private queue per worker
        let (tx, rx) = mpsc::channel::<u32>();
        senders.push(tx);
        handles.push(thread::spawn(move || {
            for task in rx {
                println!("worker {id} handles task {task}");
            }
        }));
    }

    for task in 0..16u32 {
        // Dispatcher decides the target up front, blind to current worker load
        let target = task as usize % senders.len();
        senders[target].send(task).unwrap();
    }

    drop(senders); // close queues so workers exit
    for h in handles {
        h.join().unwrap();
    }
}
