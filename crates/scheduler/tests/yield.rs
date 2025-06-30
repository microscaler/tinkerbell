use scheduler::{Scheduler, SystemCall, task::TaskContext};
use serial_test::file_serial;

#[test]
#[file_serial]
fn yield_order() {
    let mut sched = Scheduler::new();
    let a = unsafe {
        sched.spawn(|ctx: TaskContext| {
            ctx.yield_now();
            ctx.syscall(SystemCall::Done);
        })
    };

    let b = unsafe {
        sched.spawn(|ctx: TaskContext| {
            ctx.syscall(SystemCall::Done);
        })
    };

    let order = sched.run();
    assert_eq!(order, vec![b, a]);
}
