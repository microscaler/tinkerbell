use may::coroutine::{Coroutine, JoinHandle};
use crate::TaskId;

pub struct Task {
    pub tid: TaskId,
    pub handle: JoinHandle<()>,
}

impl Task {
    pub fn new<F: FnOnce() + Send + 'static>(tid: TaskId, f: F) -> Self {
        let handle = Coroutine::spawn(f);
        Self { tid, handle }
    }

    pub fn is_finished(&self) -> bool {
        self.handle.is_finished()
    }
}
