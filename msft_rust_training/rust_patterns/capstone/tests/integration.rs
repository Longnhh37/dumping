use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;

use task_scheduler::{Scheduler, SchedulerError, WorkItem};

#[test]
fn happy_path() {
    let scheduler = Scheduler::<String>::new(4);
    let mut ids = Vec::new();

    for i in 0..10 {
        let item = WorkItem::new(format!("task-{i}"), move || Ok(format!("result-{i}")));
        ids.push(scheduler.submit(item).unwrap());
    }

    let results = scheduler.shutdown();
    assert_eq!(results.len(), 10);
    assert!(results.iter().all(|r| r.is_ok()));

    let returned: Vec<_> = results.iter().map(|r| r.id()).collect();
    assert_eq!(returned, ids);
}

#[test]
fn handles_failures() {
    let scheduler = Scheduler::<String>::new(2);

    scheduler.submit(WorkItem::new("good", || Ok("ok".into()))).unwrap();
    scheduler.submit(WorkItem::new("bad", || Err("nope".into()))).unwrap();

    let results = scheduler.shutdown();
    assert_eq!(results.len(), 2);

    let failed = results.into_iter().find(|r| r.is_err()).unwrap();
    let err = failed.into_result().unwrap_err();
    assert!(matches!(err, SchedulerError::TaskFailed(_, ref m) if m == "nope"));
}

#[test]
fn empty_scheduler_shuts_down_cleanly() {
    let scheduler = Scheduler::<()>::new(4);
    assert!(scheduler.shutdown().is_empty());
}

#[test]
fn submit_after_close_is_rejected() {
    let mut scheduler = Scheduler::<()>::new(1);
    scheduler.close();

    let err = scheduler.submit(WorkItem::new("late", || Ok(()))).unwrap_err();
    assert!(matches!(err, SchedulerError::ShutDown));
}

#[test]
fn panicking_task_is_captured_as_failure() {
    let scheduler = Scheduler::<u32>::new(2);

    scheduler
        .submit(WorkItem::new("boom", || -> Result<u32, String> { panic!("kaboom") }))
        .unwrap();
    scheduler.submit(WorkItem::new("fine", || Ok(1))).unwrap();

    let results = scheduler.shutdown();
    assert_eq!(results.len(), 2);

    let boom = results.iter().find(|r| r.name() == "boom").unwrap();
    assert!(matches!(boom.outcome(), Err(msg) if msg.contains("kaboom")));
}

#[test]
fn workers_run_in_parallel() {
    let current = Arc::new(AtomicUsize::new(0));
    let peak = Arc::new(AtomicUsize::new(0));
    let scheduler = Scheduler::<()>::new(4);

    for i in 0..8 {
        let (current, peak) = (Arc::clone(&current), Arc::clone(&peak));
        scheduler
            .submit(WorkItem::new(format!("p{i}"), move || {
                let now = current.fetch_add(1, Ordering::SeqCst) + 1;
                peak.fetch_max(now, Ordering::SeqCst);
                thread::sleep(Duration::from_millis(50));
                current.fetch_sub(1, Ordering::SeqCst);
                Ok(())
            }))
            .unwrap();
    }

    scheduler.shutdown();
    assert!(peak.load(Ordering::SeqCst) >= 2);
}
