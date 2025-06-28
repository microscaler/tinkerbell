use crossbeam::channel::{Receiver, RecvTimeoutError, Sender, unbounded};
use may::coroutine::JoinHandle;
use std::collections::HashMap;
use std::time::Duration;

use crate::ready_queue::ReadyQueue;
use crate::syscall::SystemCall;
use crate::task::{Task, TaskContext, TaskId};
use crate::wait_map::WaitMap;

pub struct Scheduler {
    next_id: TaskId,
    syscall_tx: Sender<(TaskId, SystemCall)>,
    syscall_rx: Receiver<(TaskId, SystemCall)>,
    io_tx: Sender<u64>,
    io_rx: Receiver<u64>,
    tasks: HashMap<TaskId, Task>,
    ready: ReadyQueue,
    wait_map: WaitMap,
}

impl Scheduler {
    /// Create a new Scheduler instance.
    pub fn new() -> Self {
        let (syscall_tx, syscall_rx) = unbounded();
        let (io_tx, io_rx) = unbounded();
        Self {
            next_id: 1,
            syscall_tx,
            syscall_rx,
            io_tx,
            io_rx,
            tasks: HashMap::new(),
            ready: ReadyQueue::new(),
            wait_map: WaitMap::new(),
        }
    }

    /// Return a handle that can be used to signal I/O readiness.
    pub fn io_handle(&self) -> Sender<u64> {
        self.io_tx.clone()
    }

    /// Return the number of tasks currently in the ready queue.
    pub fn ready_len(&self) -> usize {
        self.ready.len()
    }

    /// Check if the ready queue is empty.
    pub fn ready_is_empty(&self) -> bool {
        self.ready.is_empty()
    }

    /// Spawn a new coroutine task with a TaskContext.
    ///
    /// # Safety
    /// This function uses `may::coroutine::spawn`, which is unsafe because it may break Rust's safety guarantees
    /// if the spawned coroutine accesses data that is not properly synchronized or outlives its stack frame.
    /// The caller must ensure that the closure and its captured data are safe to use in this context.
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

        let handle: JoinHandle<()> = unsafe { may::coroutine::spawn(move || f(ctx)) };

        self.tasks.insert(tid, Task { tid, handle });
        self.ready.push(tid);
        tid
    }

    /// Run the scheduler loop, processing system calls from tasks.
    pub fn run(&mut self) -> Vec<TaskId> {
        let mut done_order = Vec::new();
        while !self.tasks.is_empty() {
            // process any pending I/O completions
            while let Ok(io_id) = self.io_rx.try_recv() {
                for tid in self.wait_map.complete_io(io_id) {
                    self.ready.push(tid);
                }
            }

            let _tid = match self.ready.pop() {
                Some(id) => id,
                None => match self.io_rx.recv_timeout(Duration::from_secs(5)) {
                    Ok(io_id) => {
                        for tid in self.wait_map.complete_io(io_id) {
                            self.ready.push(tid);
                        }
                        continue;
                    }
                    Err(_) => break,
                },
            };

            match self.syscall_rx.recv_timeout(Duration::from_secs(5)) {
                Ok((tid, syscall)) => {
                    let mut requeue = true;
                    match syscall {
                        SystemCall::Log(msg) => tracing::info!(task = %tid, "{}", msg),
                        SystemCall::Sleep(dur) => {
                            tracing::info!(task = %tid, "sleeping {:?}", dur);
                            std::thread::sleep(dur);
                        }
                        SystemCall::Done => {
                            tracing::info!(task = %tid, "task done");
                            if let Some(task) = self.tasks.remove(&tid) {
                                task.handle.join().expect("task join failed");
                            }
                            for waiter in self.wait_map.complete(tid) {
                                self.ready.push(waiter);
                            }
                            done_order.push(tid);
                            requeue = false;
                        }
                        SystemCall::Join(target) => {
                            if self.tasks.contains_key(&target) {
                                self.wait_map.wait_for(target, tid);
                                requeue = false;
                            }
                        }
                        SystemCall::IoWait(io_id) => {
                            self.wait_map.wait_io(io_id, tid);
                            requeue = false;
                        }
                    }
                    if requeue && self.tasks.contains_key(&tid) {
                        self.ready.push(tid);
                    }
                }
                Err(RecvTimeoutError::Timeout) => {
                    tracing::warn!("scheduler idle timeout");
                    break;
                }
                Err(RecvTimeoutError::Disconnected) => break,
            }
        }
        done_order
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}
