use scheduler::{Scheduler, SystemCall, task::TaskContext};
use serial_test::file_serial;
use std::time::Duration;

#[test]
#[file_serial]
fn priority_order() {
    let mut sched = Scheduler::new();
    // Spawn tasks then run scheduler on current thread
    let _high = unsafe {
        sched.spawn_with_priority(5, |ctx: TaskContext| {
            ctx.syscall(SystemCall::Done);
        })
    };
    let _low = unsafe {
        sched.spawn_with_priority(20, |ctx: TaskContext| {
            ctx.syscall(SystemCall::Sleep(Duration::from_millis(1)));
            ctx.syscall(SystemCall::Done);
        })
    };
    let order = sched.run();
    println!("order: {order:?}");
    assert_eq!(order.first().copied(), Some(1));
    assert_eq!(order.last().copied(), Some(2));
}
