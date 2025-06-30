use scheduler::{Scheduler, SystemCall, task::TaskContext};

#[test]
fn join_wake_before_next_ready() {
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

    let _third = unsafe {
        sched.spawn(|ctx: TaskContext| {
            ctx.syscall(SystemCall::Done);
        })
    };

    let order = sched.run();
    let pos_child = order.iter().position(|&id| id == child).unwrap();
    let pos_parent = order.iter().position(|&id| id == parent).unwrap();
    assert!(
        pos_child < pos_parent,
        "child should complete before parent"
    );
}
