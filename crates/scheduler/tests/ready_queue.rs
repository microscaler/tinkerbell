use scheduler::ReadyQueue;

#[test]
fn test_ready_queue_fifo() {
    let mut q = ReadyQueue::new();
    q.push(1);
    q.push(2);
    assert_eq!(q.len(), 2);
    assert_eq!(q.pop(), Some(1));
    assert_eq!(q.pop(), Some(2));
    assert!(q.is_empty());
}
