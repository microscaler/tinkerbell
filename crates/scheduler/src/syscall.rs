use std::time::Duration;
use crate::TaskId;

/// Represents a system call yielded by a coroutine task.
#[derive(Debug, Clone)]
pub enum SystemCall {
    /// Print a log message
    Log(String),

    /// Sleep for the given duration (blocking for now)
    Sleep(Duration),

    /// Wait for another task to finish
    Join(TaskId),

    /// Signal that the task is complete
    Done,
}
