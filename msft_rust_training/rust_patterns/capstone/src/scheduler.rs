//! The concurrent scheduler: a fixed pool of workers pulling from one channel.

use std::any::Any;
use std::panic::{self, AssertUnwindSafe};
use std::sync::{Arc, Mutex, mpsc};
use std::thread::{self, JoinHandle};

use crate::error::SchedulerError;
use crate::id::TaskId;
use crate::task::{Completed, Failed, Task};
use crate::work::WorkItem;

/// The outcome of one executed task.
///
/// # Invariant
/// Fields are private and the only constructors are the 'From' impls below,
/// so a 'TaskResult' can only be built from a task that actually finished
/// ('Task<Completed<R>>' or 'Task<Failed>').
#[derive(Debug)]
pub struct TaskResult<R> {
    id: TaskId,
    name: String,
    outcome: Result<R, String>,
}

impl<R> TaskResult<R> {
    /// ID of the task that produced this result.
    pub fn id(&self) -> TaskId {
        self.id
    }

    /// Name of the task that produced this result.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The task's outcome: its value, or its error message.
    pub fn outcome(&self) -> &Result<R, String> {
        &self.outcome
    }

    /// 'true' if the task completed successfully.
    pub fn is_ok(&self) -> bool {
        self.outcome.is_ok()
    }

    /// 'true' if the task failed.
    pub fn is_err(&self) -> bool {
        self.outcome.is_err()
    }

    /// Converts a failed outcome into a ['SchedulerError::TaskFailed'].
    pub fn into_result(self) -> Result<R, SchedulerError> {
        let Self { id, outcome, .. } = self;
        outcome.map_err(|msg| SchedulerError::TaskFailed(id, msg))
    }
}

impl<R> From<Task<Completed<R>>> for TaskResult<R> {
    fn from(value: Task<Completed<R>>) -> Self {
        let (id, name, value) = value.into_parts();
        Self {
            id,
            name,
            outcome: Ok(value),
        }
    }
}

impl<R> From<Task<Failed>> for TaskResult<R> {
    fn from(value: Task<Failed>) -> Self {
        let (id, name, error) = value.into_parts();
        Self {
            id,
            name,
            outcome: Err(error),
        }
    }
}

/// A thread pool that runs ['WorkItem']s and collects their ['TaskResult']s.
///
/// # Invariant
/// - 'sender' is 'Some' until the scheduler is closed. Closing it drop the sender, which
///   makes every worker's 'recv()' fail and the workers exit.
/// - A panic inside a task is caught and reported as a failed task, so a worker thread
///   never dies because of user code.
///
/// Dropping the scheduler without calling ['Scheduler::shutdown'] still waits for
/// all queued tasks to finish (results are discarded).
///
/// ```
/// use crate::{Scheduler, WorkItem};
///
/// let scheduler = Scheduler::<u32>::new(2);
/// scheduler.submit(WorkItem::new("answer", || Ok(42))).unwrap();
///
/// let results = scheduler.shutdown();
/// assert_eq!(results[0].outcome(), &Ok(42));
/// ```
pub struct Scheduler<R: Send + 'static> {
    sender: Option<mpsc::Sender<WorkItem<R>>>,
    results: mpsc::Receiver<TaskResult<R>>,
    workers: Vec<JoinHandle<()>>,
}

impl<R: Send + 'static> Scheduler<R> {
    /// Spawn 'num_workers' worker threads (at least 1).
    pub fn new(num_workers: usize) -> Self {
        let num_workers = num_workers.max(1);

        let (work_tx, work_rx) = mpsc::channel::<WorkItem<R>>();
        let (result_tx, result_rx) = mpsc::channel::<TaskResult<R>>();

        // 'mpsc::Receiver' is not 'Clone', so workers share it behind a mutex.
        let work_rx = Arc::new(Mutex::new(work_rx));

        let workers = (0..num_workers)
            .map(|i| {
                let rx = Arc::clone(&work_rx);
                let tx = result_tx.clone();
                thread::Builder::new()
                    .name(format!("worker-{i}"))
                    .spawn(move || worker_loop(rx, tx))
                    .expect("failed to spawn worker thread")
            })
            .collect();

        // The original 'result_tx' is dropped here, so the result channel closes
        // as soon as every worker exited.
        Self {
            sender: Some(work_tx),
            results: result_rx,
            workers,
        }
    }

    /// Number of worker threads that are still owned by this scheduler.
    pub fn num_workers(&self) -> usize {
        self.workers.len()
    }

    /// Queue a task and return its ID.
    pub fn submit(&self, item: WorkItem<R>) -> Result<TaskId, SchedulerError> {
        let id = item.id();
        let sender = self.sender.as_ref().ok_or(SchedulerError::ShutDown)?;
        sender
            .send(item)
            .map_err(|_| SchedulerError::ChannelClosed)?;
        Ok(id)
    }

    /// Stops accepting new tasks. Already queued tasks still run.
    pub fn close(&mut self) {
        self.sender = None;
    }

    /// Close the queue, waits for all tasks to finish and return every result,
    /// sorted by task ID (creation order).
    pub fn shutdown(mut self) -> Vec<TaskResult<R>> {
        self.join_workers();

        // All workers have exited, so every result is already buffered.
        let mut resutls: Vec<_> = self.results.try_iter().collect();
        resutls.sort_by_key(|r| r.id());
        resutls
    }

    /// Idempotent: safe to call from both 'shutdown' or 'Drop'.
    fn join_workers(&mut self) {
        self.sender = None;
        for handle in self.workers.drain(..) {
            // Tasks run under 'catch_unwind', so a join error would mean a bug
            // in the worker loop itself. Results already sent stay collected.
            let _ = handle.join();
        }
    }
}

impl<R: Send + 'static> Drop for Scheduler<R> {
    fn drop(&mut self) {
        self.join_workers();
    }
}

fn worker_loop<R: Send + 'static>(
    rx: Arc<Mutex<mpsc::Receiver<WorkItem<R>>>>,
    result_tx: mpsc::Sender<TaskResult<R>>,
) {
    loop {
        // Hold the lock only while waiting for the next item. If we kept it
        // while running the task, the workers would execute one at a time.
        let next = {
            let guard = rx.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
            guard.recv()
        };

        let Ok(item) = next else {
            break; // Channel closed and drained: time to exit.
        };

        if result_tx.send(run(item)).is_err() {
            break; // Nobody is collecting results anymore.
        }
    }
}

/// Runs one item and drives its typestate:
/// 'Pending' -> Running -> Completed<R> | Failed'.
///
/// The value (or error) is handed to the transition itself, and the result is
/// built from the finished task, never from loose fields.
fn run<R: Send + 'static>(item: WorkItem<R>) -> TaskResult<R> {
    let (task, job) = item.into_parts();
    let running = task.start();

    let outcome = match panic::catch_unwind(AssertUnwindSafe(job)) {
        Ok(result) => result,
        Err(payload) => Err(format!(
            "task panicked: {}",
            panic_message(payload.as_ref())
        )),
    };

    match outcome {
        Ok(value) => TaskResult::from(running.complete(value)),
        Err(error) => TaskResult::from(running.fail(error)),
    }
}

fn panic_message(payload: &(dyn Any + Send)) -> &str {
    if let Some(s) = payload.downcast_ref::<&'static str>() {
        s
    } else if let Some(s) = payload.downcast_ref::<String>() {
        s.as_str()
    } else {
        "unknown panic payload"
    }
}
