use crossbeam::channel::{Receiver, RecvTimeoutError, Sender, unbounded};
use may::coroutine::JoinHandle;
use std::collections::HashMap;
use std::time::Duration;

use crate::syscall::SystemCall;
use crate::task::{Task, TaskContext, TaskId};

pub struct Scheduler {
    next_id: TaskId,
    syscall_tx: Sender<(TaskId, SystemCall)>,
    syscall_rx: Receiver<(TaskId, SystemCall)>,
    tasks: HashMap<TaskId, Task>,
}

impl Scheduler {
    /// Create a new Scheduler instance.
    pub fn new() -> Self {
        let (syscall_tx, syscall_rx) = unbounded();
        Self {
            next_id: 1,
            syscall_tx,
            syscall_rx,
            tasks: HashMap::new(),
        }
    }

    /// Spawn a new coroutine task with a TaskContext.
    pub unsafe fn spawn<F>(&mut self, f: F) -> TaskId
    where
        F: FnOnce(TaskContext) + Send + 'static,
    {
        let tid = self.next_id;
        self.next_id += 1;

        let ctx = TaskContext {
            tid,
            syscall_tx: self.syscall_tx.clone(),
        };

        let handle: JoinHandle<()> = may::coroutine::spawn(move || f(ctx));

        self.tasks.insert(tid, Task { tid, handle });
        tid
    }

    /// Run the scheduler loop, processing system calls from tasks.
    pub fn run(&mut self) {
        loop {
            match self.syscall_rx.recv_timeout(Duration::from_secs(5)) {
                Ok((tid, syscall)) => {
                    match syscall {
                        SystemCall::Log(msg) => tracing::info!(task = %tid, "{}", msg),
                        SystemCall::Sleep(dur) => {
                            tracing::info!(task = %tid, "sleeping {:?}", dur);
                            std::thread::sleep(dur);
                        }
                        SystemCall::Done => {
                            tracing::info!(task = %tid, "task done");
                            self.tasks.remove(&tid);
                        }
                        SystemCall::Join(_) => {
                            // TODO: implement join logic
                        }
                    }
                }
                Err(RecvTimeoutError::Timeout) => {
                    tracing::warn!("scheduler idle timeout");
                    break;
                }
                Err(RecvTimeoutError::Disconnected) => break,
            }

            if self.tasks.is_empty() {
                tracing::info!("all tasks complete");
                break;
            }
        }
    }
}
