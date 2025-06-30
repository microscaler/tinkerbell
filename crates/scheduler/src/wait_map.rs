use std::collections::HashMap;

use crate::task::TaskId;

/// Map of tasks waiting on other tasks to complete.
#[derive(Default)]
pub struct WaitMap {
    join_waiters: HashMap<TaskId, Vec<TaskId>>, // target -> waiting tasks
    io_waiters: HashMap<u64, Vec<TaskId>>,      // source_id -> waiting tasks
}

impl WaitMap {
    /// Create a new empty `WaitMap`.
    pub fn new() -> Self {
        Self {
            join_waiters: HashMap::new(),
            io_waiters: HashMap::new(),
        }
    }

    /// Record that `waiter` is waiting for `target` to finish.
    pub fn wait_for(&mut self, target: TaskId, waiter: TaskId) {
        self.join_waiters.entry(target).or_default().push(waiter);
    }

    /// Notify tasks waiting on `target`, returning the list of waiters.
    pub fn complete(&mut self, target: TaskId) -> Vec<TaskId> {
        self.join_waiters.remove(&target).unwrap_or_default()
    }

    /// Record that `waiter` is waiting for the I/O resource `source_id`.
    pub fn wait_io(&mut self, source_id: u64, waiter: TaskId) {
        self.io_waiters.entry(source_id).or_default().push(waiter);
    }

    /// Notify tasks waiting on an I/O resource.
    pub fn complete_io(&mut self, source_id: u64) -> Vec<TaskId> {
        self.io_waiters.remove(&source_id).unwrap_or_default()
    }
}
