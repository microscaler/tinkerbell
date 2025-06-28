use std::collections::{VecDeque, HashMap};
use std::time::{Duration, Instant};

use crate::{SystemCall, Task, TaskId};
use crate::clock::TickClock;
use tracing::{instrument};

pub struct Scheduler {
    next_id: TaskId,
    ready: VecDeque<Task>,
    sleeping: HashMap<TaskId, Instant>,
    waiting: HashMap<TaskId, TaskId>,
    tasks: HashMap<TaskId, Task>,
    clock: TickClock,
}

#[instrument(skip_all, fields(task_id = task.tid))]
fn run_task(&mut self, mut task: Task) {
    if let Some(syscall) = task.resume() {
        syscall.handle(&mut task, self);
    }

    if !self.sleeping.contains_key(&task.tid)
        && !self.waiting.contains_key(&task.tid)
    {
        self.ready.push_back(task);
    }
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            ready: VecDeque::new(),
            sleeping: HashMap::new(),
            waiting: HashMap::new(),
            tasks: HashMap::new(),
            clock: TickClock::new(Instant::now()),
        }
    }

    pub fn spawn(&mut self, generator: Box<dyn Generator<Yield = Box<dyn SystemCall>, Return = ()>>) -> TaskId {
        let tid = self.next_id;
        self.next_id += 1;
        let task = Task::new(tid, generator);
        self.tasks.insert(tid, task);
        self.ready.push_back(self.tasks.get_mut(&tid).unwrap().to_owned());
        tid
    }

    pub fn sleep(&mut self, tid: TaskId, duration: Duration) {
        self.sleeping.insert(tid, Instant::now() + duration);
    }

    pub fn wait_for(&mut self, tid: TaskId, wait_on: TaskId) {
        self.waiting.insert(tid, wait_on);
    }

    pub fn run(&mut self) {
        if let Some(task) = self.ready.pop_front() {
            self.run_task(task);
        }
        while !self.ready.is_empty() || !self.sleeping.is_empty() {
            let now = Instant::now();
            let woken: Vec<TaskId> = self.sleeping
                .iter()
                .filter(|(_, &time)| time <= now)
                .map(|(&tid, _)| tid)
                .collect();

            for tid in woken {
                if let Some(task) = self.tasks.remove(&tid) {
                    self.ready.push_back(task);
                    self.sleeping.remove(&tid);
                }
            }

            if let Some(mut task) = self.ready.pop_front() {
                if let Some(syscall) = task.resume() {
                    syscall.handle(&mut task, self);
                }

                if !self.sleeping.contains_key(&task.tid) && !self.waiting.contains_key(&task.tid) {
                    self.ready.push_back(task);
                }
            }
        }

    }
}
