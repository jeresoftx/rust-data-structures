use rust_data_structures::queue::Queue;

fn main() {
    let mut queue = Queue::with_capacity(3);

    queue.enqueue("A");
    queue.enqueue("B");
    queue.enqueue("C");
    queue.dequeue();
    queue.dequeue();
    queue.enqueue("D");
    queue.enqueue("E");

    println!("capacidad: {}", queue.capacity());
    println!(
        "orden logico: {:?}",
        queue.iter().copied().collect::<Vec<_>>()
    );
}
