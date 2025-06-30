use scheduler::task::TaskContext;
use scheduler::{Scheduler, SystemCall};
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::Duration;

#[test]
#[file_serial]
fn test_done_order() {
    let mut sched = Scheduler::new();
    let barrier = Arc::new(Barrier::new(2));
    let order = thread::scope(|s| {
        let handle = unsafe { sched.start(s, barrier.clone()) };

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

        barrier.wait();
        handle.join().unwrap()
    });
    assert_eq!(order, vec![2, 1]);
    assert_eq!(sched.ready_len(), 0);
}
