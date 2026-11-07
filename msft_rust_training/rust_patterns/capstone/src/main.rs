use std::thread;
use std::time::{Duration, Instant};

use task_scheduler::{Scheduler, WorkItem};

fn main() {
    let scheduler = Scheduler::<String>::new(4);
    let started = Instant::now();

    for i in 0..20u64 {
        let item = WorkItem::new(format!("compute-{i}"), move || {
            // Simulate work.
            thread::sleep(Duration::from_millis(10));
            if i.is_multiple_of(7) {
                Err(format!("task {i} hit a simulated error"))
            } else {
                Ok(format!("task {i} completed with value {}", i * i))
            }
        });

        if let Err(e) = scheduler.submit(item) {
            eprintln!("submit failed: {e}");
        }
    }

    println!("All tasks submitted. Shutting down...");
    let results = scheduler.shutdown();
    let elapsed = started.elapsed();

    let (ok, err): (Vec<_>, Vec<_>) = results.iter().partition(|r| r.is_ok());

    println!("\nSucceeded: {}", ok.len());
    for r in &ok {
        if let Ok(value) = r.outcome() {
            println!("\t{} -> {value}", r.name());
        }
    }

    println!("\nFailed: {}", err.len());
    for r in &err {
        if let Err(msg) = r.outcome() {
            println!("\t{} -> {msg}", r.name());
        }
    }

    println!("\nElapsed: {elapsed:?}");
}
