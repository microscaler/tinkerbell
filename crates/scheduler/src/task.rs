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
        // Yield after sending the system call so the scheduler can
        // immediately process it. We prefer `may::coroutine::yield_now` when
        // running inside a coroutine, but fall back to the standard thread
        // yield if called from a normal thread context.
        if may::coroutine::is_coroutine() {
            may::coroutine::yield_now();
        } else {
            std::thread::yield_now();
        }
    }
}
