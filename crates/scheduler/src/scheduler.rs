use std::collections::{VecDeque, HashMap};
use crate::{Task, TaskId};

pub struct Scheduler {
    next_id: TaskId,
    ready: VecDeque<Task>,
    tasks: HashMap<TaskId, Task>,
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            next_id: 1,
            ready: VecDeque::new(),
            tasks: HashMap::new(),
        }
    }

    pub fn spawn<F: FnOnce() + Send + 'static>(&mut self, f: F) -> TaskId {
        let tid = self.next_id;
        self.next_id += 1;
        let task = Task::new(tid, f);
        self.tasks.insert(tid, task);
        self.ready.push_back(self.tasks.get(&tid).unwrap().to_owned());
        tid
    }

    pub fn run(&mut self) {
        while let Some(task) = self.ready.pop_front() {
            if task.is_finished() {
                tracing::info!("Task {} finished", task.tid);
                continue;
            }
            task.handle.join().unwrap();
        }
    }
}
