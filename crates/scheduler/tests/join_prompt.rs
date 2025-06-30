use scheduler::{Scheduler, SystemCall, task::TaskContext};
use serial_test::serial;
use std::sync::{Arc, Barrier};
use std::thread;

#[test]
#[serial]
fn join_wake_before_next_ready() {
    let mut sched = Scheduler::new();
    let barrier = Arc::new(Barrier::new(2));
    let (child, parent, order) = thread::scope(|s| {
        let handle = unsafe { sched.start(s, barrier.clone()) };

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

        let _third = unsafe {
            sched.spawn(|ctx: TaskContext| {
                ctx.syscall(SystemCall::Done);
            })
        };

        barrier.wait();
        let order = handle.join().unwrap();
        (child, parent, order)
    });
    let pos_child = order.iter().position(|&id| id == child).unwrap();
    let pos_parent = order.iter().position(|&id| id == parent).unwrap();
    assert!(
        pos_child < pos_parent,
        "child should complete before parent",
    );
}
