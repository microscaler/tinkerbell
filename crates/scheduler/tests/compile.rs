use scheduler::Scheduler;

#[test]
fn compile() {
    let s = Scheduler::new();
    assert!(s.ready_is_empty());
}
