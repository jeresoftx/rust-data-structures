use rust_data_structures::stack::Stack;

fn main() {
    let mut undo = Stack::new();

    undo.push("escribir titulo");
    undo.push("agregar introduccion");
    undo.push("corregir ejemplo");

    println!("siguiente undo: {}", undo.peek().unwrap());
    println!("deshacer: {}", undo.pop().unwrap());
}
