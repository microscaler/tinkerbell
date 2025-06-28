use std::collections::HashMap;

use crate::task::TaskId;

/// Map of tasks waiting on other tasks to complete.
#[derive(Default)]
pub struct WaitMap {
    join_waiters: HashMap<TaskId, Vec<TaskId>>, // target -> waiting tasks
}

impl WaitMap {
    /// Create a new empty `WaitMap`.
    pub fn new() -> Self {
        Self {
            join_waiters: HashMap::new(),
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
}
