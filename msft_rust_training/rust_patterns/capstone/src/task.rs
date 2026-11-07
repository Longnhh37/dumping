//! Type-safe task lifecycle using typestate pattern.
//!
//! The lifecycle state is part of the *type*, and the terminal states carry the task's outcome:
//!
//! ```text
//! Pending<Pending> --start()--> Task<Running> --complete(value)--> Task<Completed<R>>
//!                                 |
//!                                 + ------------fail(error)------> Task<Failed>
//! ```
//!
//! Because 'complete' needs a value and 'fail' needs an error, a finished task
//! cannot exist without its outcome.

use std::fmt;

use crate::id::TaskId;

mod sealed {
    /// Prevents downstream crates from implementing 'TaskState'.
    pub trait Sealed {}
}

/// Implemented only by the four lifecycle states.
pub trait TaskState: sealed::Sealed {
    /// Human-readable lable used by 'Debug'.
    const LABEL: &'static str;
}

/// Submitted but not yet picked up by a worker.
#[derive(Debug)]
pub struct Pending;

/// A worker is executing the task.
#[derive(Debug)]
pub struct Running;

/// Finished successfully and carries the produced values.
#[derive(Debug)]
pub struct Completed<R>(R);

/// Finished with an error and carries the error message.
#[derive(Debug)]
pub struct Failed(String);

impl sealed::Sealed for Pending {}
impl sealed::Sealed for Running {}
impl<R> sealed::Sealed for Completed<R> {}
impl sealed::Sealed for Failed {}

impl TaskState for Pending {
    const LABEL: &'static str = "Pending";
}
impl TaskState for Running {
    const LABEL: &'static str = "Running";
}
impl<R> TaskState for Completed<R> {
    const LABEL: &'static str = "Completed";
}
impl TaskState for Failed {
    const LABEL: &'static str = "Failed";
}

/// A task whose lifecycle state `S` is tracked at compile time.
///
///
/// # Invariants
/// - `id` and 'name' never change across transitions.
/// - Only `Pending -> Running -> {Completed | Failed}` is expressible.
/// - A `Task<Completed<R>>` always holds an `R`. A `Task<Failed>` always holds an error message.
///
/// Invalid transitions fail to compile:
///
/// ```compile_fail
/// use crate::task::Task;
///
/// let done = Task::new("demo").start().complete(1u32);
/// done.start(); // error: no method `start` on `Task<Completed<u32>>`
/// ```
pub struct Task<S: TaskState> {
    id: TaskId,
    name: String,
    state: S,
}

// --- Available in every state ---
impl<S: TaskState> Task<S> {
    /// The task's unique ID.
    pub fn id(&self) -> TaskId {
        self.id
    }

    /// The task's name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Private helper: change state while keeping 'id' and 'name'.
    /// Not 'pub', so callers can only use the explicite transitions below.
    fn with_state<Next: TaskState>(self, state: Next) -> Task<Next> {
        Task {
            id: self.id,
            name: self.name,
            state,
        }
    }
}

// --- Only for 'Pending' ---
impl Task<Pending> {
    /// Creates a new pending task with a fresh ['TaskId'].
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: TaskId::next(),
            name: name.into(),
            state: Pending,
        }
    }

    /// A worker picks up the task.
    pub fn start(self) -> Task<Running> {
        self.with_state(Running)
    }
}

// --- Only for 'Running' ---
impl Task<Running> {
    /// Finish the task successfully with 'value'.
    pub fn complete<R>(self, value: R) -> Task<Completed<R>> {
        self.with_state(Completed(value))
    }

    /// Finish the task with an error.
    pub fn fail(self, error: impl Into<String>) -> Task<Failed> {
        self.with_state(Failed(error.into()))
    }
}

// --- Only for 'Completed' ---
impl<R> Task<Completed<R>> {
    /// The value produced by the task.
    pub fn value(&self) -> &R {
        &self.state.0
    }

    /// Consume the task and return '(id, name, value)'.
    pub fn into_parts(self) -> (TaskId, String, R) {
        (self.id, self.name, self.state.0)
    }
}

// --- Only for 'Failed' ---
impl Task<Failed> {
    /// The error message of the failed task.
    pub fn error(&self) -> &str {
        &self.state.0
    }

    /// Consume the task and return '(id, name, error)'.
    pub fn into_parts(self) -> (TaskId, String, String) {
        (self.id, self.name, self.state.0)
    }
}

// Does not require 'S: Debug', so the carried value is never printed.
impl<S: TaskState> fmt::Debug for Task<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Task")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("state", &S::LABEL)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn completed_task_carries_its_value() {
        let task = Task::new("job");
        let id = task.id();

        let done = task.start().complete(7u32);
        assert_eq!(done.value(), &7);

        let (done_id, name, value) = done.into_parts();
        assert_eq!(done_id, id);
        assert_eq!(name, "job");
        assert_eq!(value, 7);
    }

    #[test]
    fn failed_task_carries_its_error() {
        let failed = Task::new("bad").start().fail("boom");
        assert_eq!(failed.error(), "boom");
        assert!(format!("{failed:?}").contains("Failed"));

        let (_, name, error) = failed.into_parts();
        assert_eq!(name, "bad");
        assert_eq!(error, "boom");
    }

    #[test]
    fn pending_and_running_markers_are_zero_cost() {
        assert_eq!(size_of::<Pending>(), 0);
        assert_eq!(size_of::<Running>(), 0);
        assert_eq!(size_of::<Task<Pending>>(), size_of::<Task<Running>>());
    }

    #[test]
    fn finished_tasks_are_send() {
        fn assert_send<T: Send>() {}
        assert_send::<Task<Completed<String>>>();
        assert_send::<Task<Failed>>();
    }
}
