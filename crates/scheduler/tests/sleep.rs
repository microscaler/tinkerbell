use std::ops::Generator;
use std::pin::Pin;
use std::time::Duration;

use scheduler::{Scheduler, SystemCall, Task};
use traced_test::traced_test;

#[derive(Default)]
struct SleepTest;

impl SystemCall for SleepTest {
    fn handle(self: Box<Self>, _task: &mut Task, _sched: &mut Scheduler) {
        tracing::info!("SleepTest system call executed");
    }
}

#[traced_test]
#[test]
fn test_sleep_and_resume_trace() {
    let mut scheduler = Scheduler::new();

    let sleeper = || {
        tracing::info!("Task begins sleeping");
        yield Box::new(scheduler::syscall::Sleep {
            duration: Duration::from_secs(10),
        });
        tracing::info!("Task resumes after sleep");
    };

    scheduler.spawn(Box::new(Pin::from(sleeper)));
    scheduler.clock.tick(Duration::from_secs(5));
    scheduler.run(); // task should not resume

    scheduler.clock.tick(Duration::from_secs(5));
    scheduler.run(); // task should resume

    let logs = tracing::dispatcher::get_default(|d| format!("{:?}", d));
    insta::assert_snapshot!(logs);
}
