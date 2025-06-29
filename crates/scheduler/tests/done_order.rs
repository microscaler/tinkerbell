use scheduler::task::TaskContext;
use scheduler::{Scheduler, SystemCall};
use std::time::Duration;

#[test]
fn test_done_order() {
    let mut sched = Scheduler::new();

    unsafe {
        // task 1 sleeps longer so finishes second
        sched.spawn(|ctx: TaskContext| {
            std::thread::sleep(Duration::from_millis(50));
            ctx.syscall(SystemCall::Done);
        });
    }
    assert_eq!(sched.ready_len(), 1);

    unsafe {
        // task 2 finishes first
        sched.spawn(|ctx: TaskContext| {
            std::thread::sleep(Duration::from_millis(10));
            ctx.syscall(SystemCall::Done);
        });
    }
    assert_eq!(sched.ready_len(), 2);

    let order = sched.run();
    assert_eq!(order, vec![2, 1]);
    assert_eq!(sched.ready_len(), 0);
}
