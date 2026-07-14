use rust_data_structures::deque::Deque;

fn main() {
    let mut deque = Deque::new();
    let mut trace = Vec::new();

    deque.push_back("B");
    deque.push_front("A");
    deque.push_back("C");
    trace.push(deque.pop_front());
    trace.push(deque.pop_back());
    trace.push(deque.pop_front());

    assert_eq!(trace, vec![Some("A"), Some("C"), Some("B")]);
    println!("{trace:?}");
}
