//! Units of work submitted to the scheduler

use crate::id::TaskId;
use crate::task::{Pending, Task};

/// A boxed one-shot closure that computes a task's result
pub(crate) type Job<R> = Box<dyn FnOnce() -> Result<R, String> + Send + 'static>;

/// A ['Task'] in the 'Pending' state plus the closure that does the work
pub struct WorkItem;
