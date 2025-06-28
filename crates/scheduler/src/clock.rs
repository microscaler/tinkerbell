use std::time::{Duration, Instant};

#[allow(unused)]
#[derive(Clone)]
pub struct TickClock {
    now: Instant,
}

impl TickClock {
    #![allow(unused)]
    pub fn new(start: Instant) -> Self {
        Self { now: start }
    }

    pub fn now(&self) -> Instant {
        self.now
    }

    pub fn tick(&mut self, dur: Duration) {
        self.now += dur;
    }
}
