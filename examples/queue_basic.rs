use rust_data_structures::queue::Queue;

fn main() {
    let mut queue = Queue::new();

    queue.enqueue("primer estudiante");
    queue.enqueue("segundo estudiante");
    queue.enqueue("tercer estudiante");

    println!("siguiente: {}", queue.front().unwrap());
    println!("atendido: {}", queue.dequeue().unwrap());
}
