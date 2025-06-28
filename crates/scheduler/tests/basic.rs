#[test]
fn test_may_scheduler() {
    let mut sched = scheduler::Scheduler::new();
    sched.spawn(|| {
        println!("hello from may coroutine!");
    });
    sched.run();
}
