use rust_data_structures::stack::Stack;

fn main() {
    let mut history = Stack::new();
    let mut redo = Stack::new();

    history.push(Edit::new("crear archivo"));
    history.push(Edit::new("agregar vector"));
    history.push(Edit::new("agregar stack"));

    if let Some(edit) = history.pop() {
        println!("undo: {}", edit.description);
        redo.push(edit);
    }

    if let Some(edit) = redo.pop() {
        println!("redo: {}", edit.description);
        history.push(edit);
    }
}

struct Edit {
    description: &'static str,
}

impl Edit {
    fn new(description: &'static str) -> Self {
        Self { description }
    }
}
