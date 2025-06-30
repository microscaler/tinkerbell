#[test]
fn test_may_scheduler() {
    let mut sched = scheduler::Scheduler::new();
    unsafe {
        sched.spawn(|_| {
            println!("hello from may coroutine!");
        });
    }
    let _ = sched.run();
}
