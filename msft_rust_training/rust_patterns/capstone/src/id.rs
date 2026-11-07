//! Task Identifiers
//!
//! Invariant: every 'TaskId' returned by ['TaskId::next'] is unique within the
//! lifetime of the process. The inner value is private, so callers cannot forge an ID

use std::fmt;
use std::sync::atomic::{AtomicU64, Ordering};

/// Global counter shared by all threads
static NEXT_ID: AtomicU64 = AtomicU64::new(1);

/// Unique identifier of a task
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TaskId(u64);

impl TaskId {
    pub fn next() -> Self {
        Self(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }

    pub const fn get(self) -> u64 {
        self.0
    }
}

impl fmt::Display for TaskId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "task#{}", self.0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    use std::thread;

    #[test]
    fn ids_are_strictly_increasing_on_one_thread() {
        let a = TaskId::next();
        let b = TaskId::next();
        assert!(b > a);
    }

    #[test]
    fn ids_are_unique_across_thread() {
        const THREADS: usize = 8;
        const PER_THREAD: usize = 1000;

        let all: Vec<TaskId> = thread::scope(|s| {
            let handles: Vec<_> = (0..THREADS)
                .map(|_| s.spawn(|| (0..PER_THREAD).map(|_| TaskId::next()).collect::<Vec<_>>()))
                .collect();
            handles
                .into_iter()
                .flat_map(|h| h.join().expect("thread panicked"))
                .collect()
        });

        let unique: HashSet<TaskId> = all.iter().copied().collect();
        assert_eq!(unique.len(), THREADS * PER_THREAD);
    }

    #[test]
    fn display_format() {
        let id = TaskId::next();
        assert_eq!(id.to_string(), format!("task#{}", id.get()));
    }
}
