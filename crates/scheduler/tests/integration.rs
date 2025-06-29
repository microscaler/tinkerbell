use scheduler::{Scheduler, SystemCall, task::TaskContext};
use std::time::Duration;

#[test]
fn integration_task_order() {
    let mut sched = Scheduler::new();

    unsafe {
        sched.spawn(|ctx: TaskContext| {
            std::thread::sleep(Duration::from_millis(50));
            ctx.syscall(SystemCall::Done);
        });
    }

    unsafe {
        sched.spawn(|ctx: TaskContext| {
            std::thread::sleep(Duration::from_millis(10));
            ctx.syscall(SystemCall::Done);
        });
    }

    let order = sched.run();
    assert_eq!(order, vec![2, 1]);
}

#[test]
fn integration_join_and_io_wait() {
    let mut sched = Scheduler::new();
    let io_tx = sched.io_handle();

    let child = unsafe {
        sched.spawn(|ctx: TaskContext| {
            ctx.syscall(SystemCall::Done);
        })
    };

    unsafe {
        sched.spawn(move |ctx: TaskContext| {
            ctx.syscall(SystemCall::Join(child));
            ctx.syscall(SystemCall::Done);
        });
    }

    unsafe {
        sched.spawn(|ctx: TaskContext| {
            ctx.syscall(SystemCall::IoWait(1));
            ctx.syscall(SystemCall::Done);
        });
    }

    std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(20));
        io_tx.send(1).unwrap();
    });

    let order = sched.run();
    assert_eq!(order.len(), 3);
    assert_eq!(order[0], child);
    assert!(order[1..].contains(&2));
    assert!(order[1..].contains(&3));
}
