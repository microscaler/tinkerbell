use scheduler::{Scheduler, SystemCall, task::TaskContext};
use serial_test::serial;

#[test]
#[serial]
fn syscall_yield_order() {
    let mut sched = Scheduler::new();

    unsafe {
        sched.spawn(|ctx: TaskContext| {
            ctx.syscall(SystemCall::Done);
        });
    }

    unsafe {
        sched.spawn(|ctx: TaskContext| {
            ctx.syscall(SystemCall::Done);
        });
    }

    let order = sched.run();
    assert_eq!(order, vec![1, 2]);
}
