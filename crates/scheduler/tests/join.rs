use scheduler::{Scheduler, SystemCall, task::TaskContext};
use serial_test::serial;

#[test]
#[serial]
fn test_join_wakes_waiter() {
    let mut sched = Scheduler::new();
    let child = unsafe {
        sched.spawn(|ctx: TaskContext| {
            ctx.syscall(SystemCall::Done);
        })
    };

    let parent = unsafe {
        sched.spawn(move |ctx: TaskContext| {
            ctx.syscall(SystemCall::Join(child));
            ctx.syscall(SystemCall::Done);
        })
    };

    let order = sched.run();
    let (child, parent, order) = (child, parent, order);
    assert_eq!(order, vec![child, parent]);
}
