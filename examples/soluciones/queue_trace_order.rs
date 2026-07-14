use rust_data_structures::queue::Queue;

fn main() {
    let mut queue = Queue::new();
    let mut trace = Vec::new();

    queue.enqueue("A");
    queue.enqueue("B");
    trace.push(queue.dequeue());
    queue.enqueue("C");
    trace.push(queue.dequeue());
    trace.push(queue.dequeue());

    assert_eq!(trace, vec![Some("A"), Some("B"), Some("C")]);
    println!("{trace:?}");
}
