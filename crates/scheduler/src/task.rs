use crate::syscall::SystemCall;
use crossbeam::channel::Sender;

/// Unique identifier for a task.
pub type TaskId = u64;

/// A wrapper around a running coroutine and its metadata.
pub struct Task {
    pub tid: TaskId,
    pub handle: may::coroutine::JoinHandle<()>,
}

/// Shared context passed into each task.
///
/// Allows tasks to submit system calls to the scheduler.
#[derive(Clone)]
pub struct TaskContext {
    pub tid: TaskId,
    pub syscall_tx: Sender<(TaskId, SystemCall)>,
}

impl TaskContext {
    /// Submit a system call from the current task.
    pub fn syscall(&self, call: SystemCall) {
        self.syscall_tx
            .send((self.tid, call))
            .expect("Failed to send system call");
        // Yield after sending the syscall so the scheduler can handle it
        // promptly. `may::coroutine::yield_now` already falls back to
        // `std::thread::yield_now` when not in a coroutine context.
        may::coroutine::yield_now();
    }
}
