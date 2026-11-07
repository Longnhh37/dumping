//! Error types of the scheduler

use thiserror::Error;

use crate::id::TaskId;

/// Errors returned by the scheduler's public API
#[derive(Debug, Error)]
pub enum SchedulerError {
    /// The scheduler no longer accepts new tasks (see 'Scheduler::close')
    #[error("scheduler is shut down")]
    ShutDown,

    /// a task finished with an error (produced by 'TaskResult::into_result')
    #[error("task: {0} failed: {1}")]
    TaskFailed(TaskId, String),

    /// All workers exited, so the work channel is closed
    #[error("work channel is closed (all workers have exited)")]
    ChannelClosed,
}
