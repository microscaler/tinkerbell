use scheduler::Scheduler;

#[test]
fn compile() {
    let mut sched = Scheduler::new();
    let order = sched.run();
    assert!(sched.ready_is_empty());
    assert!(order.is_empty());
}
