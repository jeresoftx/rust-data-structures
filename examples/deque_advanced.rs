use rust_data_structures::deque::Deque;

fn main() {
    let mut deque = Deque::with_capacity(4);

    deque.push_back("B");
    deque.push_back("C");
    deque.push_back("D");
    deque.pop_front();
    deque.push_back("E");
    deque.push_front("A");

    println!("capacidad: {}", deque.capacity());
    println!(
        "orden logico: {:?}",
        deque.iter().copied().collect::<Vec<_>>()
    );
    println!("indice 2: {:?}", deque.get(2));
}
