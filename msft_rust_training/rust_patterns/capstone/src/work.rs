//! Units of work submitted to the scheduler

use crate::id::TaskId;
use crate::task::{Pending, Task};

/// A boxed one-shot closure that computes a task's result.
pub(crate) type Job<R> = Box<dyn FnOnce() -> Result<R, String> + Send + 'static>;

/// A [`Task`] in the `Pending` state plus the closure that does the work.
///
/// # Invariant
/// A `WorkItem` always holds a `Pending` task: the only way to advance the
/// lifecycle is for a worker to consume the item
pub struct WorkItem<R> {
    task: Task<Pending>,
    job: Job<R>,
}

impl<R: Send + 'static> WorkItem<R> {
    /// Creates a work item. A fresh unique [`TaskId`] is assigned automatically.
    pub fn new<F>(name: impl Into<String>, work: F) -> Self
    where
        F: FnOnce() -> Result<R, String> + Send + 'static,
    {
        Self {
            task: Task::new(name),
            job: Box::new(work),
        }
    }

    /// The ID of the underlying task.
    pub fn id(&self) -> TaskId {
        self.task.id()
    }

    /// The name of the underlying task.
    pub fn name(&self) -> &str {
        self.task.name()
    }

    /// Split the item so a worker can drive the typestate transitions.
    pub(crate) fn into_parts(self) -> (Task<Pending>, Job<R>) {
        (self.task, self.job)
    }
}
