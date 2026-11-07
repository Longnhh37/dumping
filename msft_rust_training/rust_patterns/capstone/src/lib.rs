#![forbid(unsafe_code)]

pub mod error;
pub mod id;
pub mod scheduler;
pub mod task;
pub mod work;

pub use error::SchedulerError;
pub use id::TaskId;
pub use task::{Completed, Failed, Pending, Running, Task};
pub use scheduler::{Scheduler, TaskResult};
pub use work::WorkItem;
