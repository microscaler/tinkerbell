use std::cell::RefCell;
use std::rc::Rc;

use crate::SystemCall;

pub type TaskId = u64;

pub struct Task {
    pub tid: TaskId,
    generator: Box<dyn Generator<Yield = Box<dyn SystemCall>, Return = ()>>,
}

impl Task {
    pub fn new(tid: TaskId, generator: Box<dyn Generator<Yield = Box<dyn SystemCall>, Return = ()>>) -> Self {
        Self { tid, generator }
    }

    pub fn resume(&mut self) -> Option<Box<dyn SystemCall>> {
        match self.generator.as_mut().resume(()) {
            std::ops::GeneratorState::Yielded(syscall) => Some(syscall),
            std::ops::GeneratorState::Complete(_) => None,
        }
    }
}
