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
        // Yield after sending the syscall to give the scheduler a chance to
        // process it immediately. This keeps task execution deterministic when
        // multiple tasks are contending for the scheduler.
        may::coroutine::yield_now();
        // Fallback to OS thread yielding for environments where the `may`
        // runtime might not actively schedule.
        std::thread::yield_now();
    }
}
