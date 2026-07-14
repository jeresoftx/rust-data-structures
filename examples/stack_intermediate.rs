use rust_data_structures::stack::Stack;

fn main() {
    let expression = "([]{})";

    if is_balanced(expression) {
        println!("{expression} esta balanceada");
    } else {
        println!("{expression} no esta balanceada");
    }
}

fn is_balanced(input: &str) -> bool {
    let mut stack = Stack::new();

    for ch in input.chars() {
        match ch {
            '(' | '[' | '{' => stack.push(ch),
            ')' if stack.pop() != Some('(') => return false,
            ']' if stack.pop() != Some('[') => return false,
            '}' if stack.pop() != Some('{') => return false,
            ')' | ']' | '}' => {}
            _ => {}
        }
    }

    stack.is_empty()
}
