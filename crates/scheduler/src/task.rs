use crate::syscall::SystemCall;
use crossbeam::channel::Sender;

/// Unique identifier for a task.
pub type TaskId = u64;

/// A wrapper around a running coroutine and its metadata.
pub struct Task {
    /// Unique identifier for the task.
    pub tid: TaskId,
    /// Scheduling priority (0 = highest).
    pub pri: u8,
    /// Coroutine handle backing the task.
    pub handle: may::coroutine::JoinHandle<()>,
    /// Current lifecycle state of the task.
    pub state: TaskState,
}

/// Represents the lifecycle state of a task.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TaskState {
    /// The task is currently running or ready to run.
    Running,
    /// The task completed successfully.
    Finished,
    /// The task terminated due to a panic.
    Failed,
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

    /// Yield back to the scheduler without performing a system call.
    pub fn yield_now(&self) {
        self.syscall(SystemCall::Yield);
    }
}
