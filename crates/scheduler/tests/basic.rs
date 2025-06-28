use scheduler::{Scheduler, SystemCall};
use std::ops::Generator;
use std::pin::Pin;

#[test]
fn test_basic_spawn() {
    fn echo_task() -> impl Generator<Yield = Box<dyn SystemCall>, Return = ()> {
        || {
            println!("Hello from coroutine!");
            yield Box::new(Noop {});
            println!("Done!");
        }
    }

    struct Noop;

    impl SystemCall for Noop {
        fn handle(self: Box<Self>, _task: &mut scheduler::Task, _sched: &mut Scheduler) {
            // No-op
        }
    }

    let mut sched = Scheduler::new();
    sched.spawn(Box::pin(echo_task()));
    sched.run();
}
