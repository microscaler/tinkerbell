use std::time::Duration;
use crate::{Scheduler, Task, TaskId};


pub struct Spawn {
    pub task: Box<dyn Generator<Yield = Box<dyn SystemCall>, Return = ()>>,
}

pub struct Sleep {
    pub duration: Duration,
}

impl SystemCall for Sleep {
    fn handle(self: Box<Self>, task: &mut Task, sched: &mut Scheduler) {
        sched.sleep(task.tid, self.duration);
    }
}

pub struct WaitTask {
    pub wait_on: TaskId,
}

pub trait SystemCall {
    fn handle(self: Box<Self>, task: &mut Task, sched: &mut Scheduler);
}


impl SystemCall for WaitTask {
    fn handle(self: Box<Self>, task: &mut Task, sched: &mut Scheduler) {
        sched.wait_for(task.tid, self.wait_on);
    }
}

impl SystemCall for Spawn {
    fn handle(self: Box<Self>, _task: &mut Task, sched: &mut Scheduler) {
        sched.spawn(self.task);
    }
}
