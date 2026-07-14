use rust_data_structures::stack::Stack;

fn main() {
    let mut stack = Stack::new();
    let mut trace = Vec::new();

    stack.push("A");
    trace.push(stack.peek().copied());

    stack.push("B");
    trace.push(stack.peek().copied());

    stack.pop();
    trace.push(stack.peek().copied());

    assert_eq!(trace, vec![Some("A"), Some("B"), Some("A")]);
    println!("{trace:?}");
}
