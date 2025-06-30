use std::collections::{HashSet, VecDeque};

use crate::task::TaskId;

/// FIFO queue of runnable task IDs.
#[derive(Default)]
pub struct ReadyQueue {
    queue: VecDeque<TaskId>,
    set: HashSet<TaskId>,
}

impl ReadyQueue {
    /// Create an empty ready queue.
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
            set: HashSet::new(),
        }
    }

    /// Push a task ID onto the queue.
    pub fn push(&mut self, tid: TaskId) {
        if self.set.insert(tid) {
            self.queue.push_back(tid);
        }
    }

    /// Returns `true` if the queue already contains `tid`.
    pub fn contains(&self, tid: TaskId) -> bool {
        self.set.contains(&tid)
    }

    /// Pop the next task ID from the queue.
    pub fn pop(&mut self) -> Option<TaskId> {
        if let Some(tid) = self.queue.pop_front() {
            self.set.remove(&tid);
            Some(tid)
        } else {
            None
        }
    }

    /// Returns `true` if the queue has no tasks.
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    /// Returns the number of tasks in the queue.
    pub fn len(&self) -> usize {
        self.queue.len()
    }
}

impl ReadyQueue {
    /// Push a task ID without checking for duplicates. Used only for tests.
    pub fn force_push(&mut self, tid: TaskId) {
        self.queue.push_back(tid);
    }
}
