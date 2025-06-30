use scheduler::{Scheduler, SystemCall, task::TaskContext};
use serial_test::serial;
use std::sync::{Arc, Barrier};
use std::thread;

#[test]
#[serial]
fn test_join_wakes_waiter() {
    let mut sched = Scheduler::new();
    // main thread + scheduler thread + two tasks
    let barrier = Arc::new(Barrier::new(4));
    let (child, parent, order) = thread::scope(|s| {
        let handle = unsafe { sched.start(s, barrier.clone()) };

        let b = barrier.clone();
        let child = unsafe {
            sched.spawn(move |ctx: TaskContext| {
                b.wait();
                ctx.syscall(SystemCall::Done);
            })
        };

        let b = barrier.clone();
        let parent = unsafe {
            sched.spawn(move |ctx: TaskContext| {
                b.wait();
                ctx.syscall(SystemCall::Join(child));
                ctx.syscall(SystemCall::Done);
            })
        };

        barrier.wait();
        let order = handle.join().unwrap();
        assert_eq!(order, vec![child, parent]);
        (child, parent, order)
    });
    assert_eq!(order, vec![child, parent]);
}
