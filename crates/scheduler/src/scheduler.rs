use crossbeam::channel::{Receiver, RecvTimeoutError, Sender, unbounded};
use may::coroutine::JoinHandle;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::sync::{Arc, Barrier};
use std::thread::{Scope, ScopedJoinHandle};
use std::time::{Duration, Instant};

#[cfg(feature = "async-io")]
use mio::{Events, Interest, Poll, Token, unix::SourceFd};

use crate::clock::TickClock;
#[cfg(feature = "async-io")]
use crate::io::IoSource;
use crate::ready_queue::ReadyQueue;
use crate::syscall::SystemCall;
use crate::task::{Task, TaskContext, TaskId};
use crate::wait_map::WaitMap;

/// Core runtime orchestrator managing runnable tasks, pending I/O events,
/// and join waiters.
pub struct Scheduler {
    next_id: TaskId,
    syscall_tx: Sender<(TaskId, SystemCall)>,
    syscall_rx: Receiver<(TaskId, SystemCall)>,
    #[cfg(not(feature = "async-io"))]
    io_tx: Sender<u64>,
    #[cfg(not(feature = "async-io"))]
    io_rx: Receiver<u64>,
    #[cfg(feature = "async-io")]
    poll: Poll,
    #[cfg(feature = "async-io")]
    sources: HashMap<Token, Arc<dyn IoSource>>,
    #[cfg(feature = "async-io")]
    next_token: usize,
    clock: TickClock,
    sleepers: BinaryHeap<Reverse<(Instant, TaskId)>>,
    tasks: HashMap<TaskId, Task>,
    ready: ReadyQueue,
    wait_map: WaitMap,
}

impl Scheduler {
    /// Create a new Scheduler instance.
    pub fn new() -> Self {
        let (syscall_tx, syscall_rx) = unbounded();
        #[cfg(not(feature = "async-io"))]
        let (io_tx, io_rx) = unbounded();
        #[cfg(feature = "async-io")]
        let poll = Poll::new().expect("poll");
        Self {
            next_id: 1,
            syscall_tx,
            syscall_rx,
            #[cfg(not(feature = "async-io"))]
            io_tx,
            #[cfg(not(feature = "async-io"))]
            io_rx,
            #[cfg(feature = "async-io")]
            poll,
            #[cfg(feature = "async-io")]
            sources: HashMap::new(),
            #[cfg(feature = "async-io")]
            next_token: 0,
            clock: TickClock::new(Instant::now()),
            sleepers: BinaryHeap::new(),
            tasks: HashMap::new(),
            ready: ReadyQueue::new(),
            wait_map: WaitMap::new(),
        }
    }

    /// Return a handle that can be used to signal I/O readiness when
    /// `async-io` is disabled.
    #[cfg(not(feature = "async-io"))]
    pub fn io_handle(&self) -> Sender<u64> {
        self.io_tx.clone()
    }

    /// Register a new I/O source with the scheduler.
    #[cfg(feature = "async-io")]
    pub fn register_io(&mut self, src: Arc<dyn IoSource>) {
        let token = Token(self.next_token);
        self.next_token += 1;
        let fd = src.raw_fd();
        let mut source = SourceFd(&fd);
        self.poll
            .registry()
            .register(&mut source, token, Interest::READABLE)
            .expect("register io source");
        self.sources.insert(token, src);
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
    #[cfg(not(feature = "async-io"))]
    pub fn run(&mut self) -> Vec<TaskId> {
        let mut done_order = Vec::new();
        while !self.tasks.is_empty() {
            while let Some(&Reverse((wake_at, tid))) = self.sleepers.peek() {
                if wake_at <= self.clock.now() {
                    self.sleepers.pop();
                    self.ready.push(tid);
                } else {
                    break;
                }
            }

            while let Ok((call_tid, syscall)) = self.syscall_rx.try_recv() {
                self.handle_syscall(call_tid, syscall, &mut done_order);
            }

            while let Ok(io_id) = self.io_rx.try_recv() {
                for tid in self.wait_map.complete_io(io_id) {
                    self.ready.push(tid);
                }
            }

            let tid = match self.ready.pop() {
                Some(id) => id,
                None => {
                    if let Some(&Reverse((wake_at, _))) = self.sleepers.peek() {
                        if wake_at > self.clock.now() {
                            let diff = wake_at.duration_since(self.clock.now());
                            self.clock.tick(diff);
                        }
                        continue;
                    }
                    match self.io_rx.recv_timeout(Duration::from_secs(5)) {
                        Ok(io_id) => {
                            for tid in self.wait_map.complete_io(io_id) {
                                self.ready.push(tid);
                            }
                            continue;
                        }
                        Err(_) => break,
                    }
                }
            };

            if !self.tasks.contains_key(&tid) {
                continue;
            }

            let _task = self.tasks.get_mut(&tid).expect("task not found");

            match self.syscall_rx.recv_timeout(Duration::from_secs(5)) {
                Ok((call_tid, syscall)) => {
                    if call_tid != tid && self.tasks.contains_key(&tid) {
                        self.ready.push(tid);
                    }
                    self.handle_syscall(call_tid, syscall, &mut done_order);
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

    #[cfg(feature = "async-io")]
    pub fn run(&mut self) -> Vec<TaskId> {
        let mut done_order = Vec::new();
        let mut events = Events::with_capacity(8);
        while !self.tasks.is_empty() {
            let timeout = if self.ready.is_empty() {
                if let Some(&Reverse((wake_at, _))) = self.sleepers.peek() {
                    let now = self.clock.now();
                    if wake_at > now {
                        wake_at - now
                    } else {
                        Duration::ZERO
                    }
                } else {
                    Duration::from_secs(5)
                }
            } else {
                Duration::ZERO
            };

            if let Err(e) = self.poll.poll(&mut events, Some(timeout)) {
                tracing::warn!(?e, "poll error");
                break;
            }

            if events.is_empty()
                && self.ready.is_empty()
                && self.sleepers.is_empty()
                && timeout == Duration::from_secs(5)
            {
                break;
            }

            if events.is_empty() && self.ready.is_empty() && timeout > Duration::ZERO {
                self.clock.tick(timeout);
            }

            for ev in events.iter() {
                if let Some(src) = self.sources.get(&ev.token()) {
                    for tid in self.wait_map.complete_io(src.id()) {
                        self.ready.push(tid);
                    }
                }
            }
            events.clear();

            while let Some(&Reverse((wake_at, tid))) = self.sleepers.peek() {
                if wake_at <= self.clock.now() {
                    self.sleepers.pop();
                    self.ready.push(tid);
                } else {
                    break;
                }
            }

            while let Ok((call_tid, syscall)) = self.syscall_rx.try_recv() {
                self.handle_syscall(call_tid, syscall, &mut done_order);
            }

            let tid = match self.ready.pop() {
                Some(id) => id,
                None => continue,
            };

            if !self.tasks.contains_key(&tid) {
                continue;
            }

            let _task = self.tasks.get_mut(&tid).expect("task not found");

            match self.syscall_rx.recv_timeout(Duration::from_secs(5)) {
                Ok((call_tid, syscall)) => {
                    if call_tid != tid && self.tasks.contains_key(&tid) {
                        self.ready.push(tid);
                    }
                    self.handle_syscall(call_tid, syscall, &mut done_order);
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

    /// Start the scheduler loop on a dedicated thread.
    ///
    /// The provided `barrier` is used to coordinate when the loop begins.
    /// Tasks should be spawned before the barrier is released.
    ///
    /// # Safety
    /// This method spawns a thread that operates on `&mut self`. The caller
    /// must ensure no other references to `self` are used once the barrier is
    /// released and until the returned handle has completed.
    pub unsafe fn start<'scope>(
        &mut self,
        scope: &'scope Scope<'scope, '_>,
        barrier: Arc<Barrier>,
    ) -> ScopedJoinHandle<'scope, Vec<TaskId>> {
        struct Ptr(*mut Scheduler);
        unsafe impl Send for Ptr {}

        let ptr = Ptr(self as *mut Scheduler);
        scope.spawn(move || {
            let p = ptr;
            barrier.wait();
            // SAFETY: exclusive access is guaranteed after the barrier.
            unsafe { &mut *p.0 }.run()
        })
    }
}

impl Default for Scheduler {
    fn default() -> Self {
        Self::new()
    }
}

impl Scheduler {
    /// Directly push a task ID into the ready queue without deduplication.
    pub fn ready_push_duplicate_for_test(&mut self, tid: TaskId) {
        self.ready.force_push(tid);
    }

    fn handle_syscall(&mut self, tid: TaskId, syscall: SystemCall, done: &mut Vec<TaskId>) {
        let mut requeue = true;
        match syscall {
            SystemCall::Log(msg) => tracing::info!(task = %tid, "{}", msg),
            SystemCall::Sleep(dur) => {
                tracing::info!(task = %tid, "sleeping {:?}", dur);
                let wake_at = self.clock.now() + dur;
                self.sleepers.push(Reverse((wake_at, tid)));
                requeue = false;
            }
            SystemCall::Done => {
                tracing::info!(task = %tid, "task done");
                if let Some(task) = self.tasks.remove(&tid) {
                    task.handle.join().expect("task join failed");
                }
                for waiter in self.wait_map.complete(tid) {
                    self.ready.push(waiter);
                }
                done.push(tid);
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
            SystemCall::Yield => {
                // Cooperative yield: no action required other than requeueing
            }
        }
        if requeue && self.tasks.contains_key(&tid) {
            self.ready.push(tid);
        }
    }
}
