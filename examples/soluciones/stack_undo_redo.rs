use rust_data_structures::stack::Stack;

fn main() {
    let mut undo = Stack::new();
    let mut redo = Stack::new();

    undo.push("crear capitulo");
    undo.push("agregar diagrama");
    undo.push("corregir benchmark");

    move_top(&mut undo, &mut redo);
    move_top(&mut redo, &mut undo);

    assert_eq!(undo.peek(), Some(&"corregir benchmark"));
    assert!(redo.is_empty());
    println!("undo/redo consistente");
}

fn move_top<T>(from: &mut Stack<T>, to: &mut Stack<T>) {
    if let Some(value) = from.pop() {
        to.push(value);
    }
}
