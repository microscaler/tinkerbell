use scheduler::{Scheduler, syscall::SystemCall, task::TaskContext};
use std::time::Duration;

#[test]
fn test_task_log_and_sleep_with_may() {
    let mut scheduler = Scheduler::new();

    unsafe {
        scheduler.spawn(|ctx: TaskContext| {
            ctx.syscall(SystemCall::Log("start task A".into()));
            ctx.syscall(SystemCall::Sleep(Duration::from_millis(100)));
            ctx.syscall(SystemCall::Log("resume task A".into()));
            ctx.syscall(SystemCall::Done);
        });
    }

    unsafe {
        scheduler.spawn(|ctx: TaskContext| {
            ctx.syscall(SystemCall::Log("start task B".into()));
            ctx.syscall(SystemCall::Sleep(Duration::from_millis(100)));
            ctx.syscall(SystemCall::Log("resume task B".into()));
            ctx.syscall(SystemCall::Done);
        });
    }

    scheduler.run();
}
