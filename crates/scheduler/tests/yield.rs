use scheduler::{Scheduler, SystemCall, task::TaskContext};
use serial_test::file_serial;

#[test]
#[file_serial]
fn yield_order() {
    let mut sched = Scheduler::new();
    let barrier = std::sync::Arc::new(std::sync::Barrier::new(2));
    let order = std::thread::scope(|s| {
        let handle = unsafe { sched.start(s, barrier.clone()) };
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
        barrier.wait();
        let order = handle.join().unwrap();
        assert_eq!(order, vec![b, a]);
        order
    });
    assert_eq!(order.len(), 2);
}
