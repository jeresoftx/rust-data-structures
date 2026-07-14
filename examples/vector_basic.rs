use rust_data_structures::vector::Vector;

fn main() {
    let mut topics = Vector::new();

    topics.push("vector");
    topics.push("linked_list");
    topics.push("stack");

    println!("primer tema: {}", topics.get(0).unwrap());
    println!("ultimo tema removido: {}", topics.pop().unwrap());
    println!("temas restantes: {}", topics.len());
}
