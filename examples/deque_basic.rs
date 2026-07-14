use rust_data_structures::deque::Deque;

fn main() {
    let mut deque = Deque::new();

    deque.push_back("centro");
    deque.push_front("inicio");
    deque.push_back("final");

    println!("frente: {}", deque.front().unwrap());
    println!("fondo: {}", deque.back().unwrap());
}
