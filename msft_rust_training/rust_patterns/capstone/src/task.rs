//! Type-safe task lifecycle using typestate pattern
//!
//! A ['Task'] carries its lifecycle state in its *type*. Transitions consume 'self' and return
//! a new type, so invalid transitions do not compile
//!
//! ```text
//! Pending --start()--> Running --complete()--> Completed
//!                         |
//!                         + -----fail()------> Failed
//! ```

use std::fmt;
use std::marker::PhantomData;

use crate::id::TaskId;

mod sealed {
    /// Prevents downstream crates from implementing 'TaskState'
    pub trait Sealed {}
}

/// Marker trait implemeted only by the four lifecycle states
pub trait TaskState: sealed::Sealed {
    const LABEL: &'static str;
}

// Uninhabited enums: zero-sized types

/// Task is submitted but not yet picked up by a worker.
#[derive(Debug)]
pub enum Pending {}
/// A worker is executing the task.
#[derive(Debug)]
pub enum Running {}
/// The task finished successfully.
#[derive(Debug)]
pub enum Completed {}
/// The task finished with an error.
#[derive(Debug)]
pub enum Failed {}

macro_rules! impl_state {
    ($($state:ident),+ $(,)?) => {
        $(
            impl sealed::Sealed for $state {}
            impl TaskState for $state {
                const LABEL: &'static str = stringify!($state);
            }
        )+
    };
}
impl_state!(Pending, Running, Completed, Failed);

/// A task whose lifecycle state `S` is tracked at compile time
///
/// `R` is the result type the task will eventually produce
///
/// # Invariants
/// - `id` never changes across transitions.
/// - Only `Pending -> Running -> {Completed | Failed}` is expressible
///
/// Invalid transitions fail to compile:
///
/// ```compile_fail
/// use crate::task::{Pending, Task};
///
/// let done = Task::<Pending, ()>::new("demo").start().complete();
/// done.start(); // error: no method `start` on `Task<Completed, ()>`
/// ```
pub struct Task<S: TaskState, R> {
    id: TaskId,
    name: String,
    _state: PhantomData<S>,
    // 'fn() -> R' keeps auto traits (Send/Sync) and variance independent of 'R':
    // a Task does not actually own an 'R', it only names its type
    _result: PhantomData<fn() -> R>,
}

// --- Available in every state ---
impl<S: TaskState, R> Task<S, R> {
    /// task's unique ID
    pub fn id(&self) -> TaskId {
        self.id
    }

    /// task's name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// private helper: change state while keeping 'id' and 'name'
    /// not 'pub', so callers can only use the explicite transitions below
    fn transition<Next: TaskState>(self) -> Task<Next, R> {
        Task {
            id: self.id,
            name: self.name,
            _state: PhantomData,
            _result: PhantomData,
        }
    }
}

// --- Only for 'Pending' ---
impl<R> Task<Pending, R> {
    /// creates a new pending task with a fresh ['TaskId']
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: TaskId::next(),
            name: name.into(),
            _state: PhantomData,
            _result: PhantomData,
        }
    }

    /// A worker picks up the task
    pub fn start(self) -> Task<Running, R> {
        self.transition()
    }
}

// --- Only for 'Running' ---
impl<R> Task<Running, R> {
    /// task succeeded
    pub fn complete(self) -> Task<Completed, R> {
        self.transition()
    }

    /// task returns an error
    pub fn fail(self) -> Task<Failed, R> {
        self.transition()
    }
}

impl<S: TaskState, R> fmt::Debug for Task<S, R> {
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
    fn happy_path_keeps_id_and_name() {
        let t = Task::<Pending, u32>::new("job");
        let id = t.id();

        let done = t.start().complete();
        assert_eq!(done.id(), id);
        assert_eq!(done.name(), "job");
    }

    #[test]
    fn failure_path() {
        let failed = Task::<Pending, ()>::new("bad").start().fail();
        assert!(format!("{failed:?}").contains("Failed"))
    }

    #[test]
    fn state_markers_are_zero_cost() {
        assert_eq!(
            size_of::<Task<Pending, String>>(),
            size_of::<Task<Running, String>>()
        );
        assert_eq!(
            size_of::<Task<Pending, String>>(),
            size_of::<Task<Completed, ()>>()
        );
        assert_eq!(size_of::<Pending>(), 0);
    }

    #[test]
    fn task_is_send_even_if_result_type_is_not() {
        fn assert_send<T: Send>() {}
        // Rc<()> is !Send, but Task<_, Rc<()>> stays Send thanks to PhantomData<fn() -> R>
        assert_send::<Task<Pending, std::rc::Rc<()>>>();
    }
}
