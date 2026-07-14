use rust_data_structures::stack::Stack;
use std::cell::Cell;
use std::rc::Rc;

#[test]
fn push_and_pop_follow_lifo_order() {
    let mut stack = Stack::new();

    assert!(stack.is_empty());
    assert_eq!(stack.len(), 0);
    assert_eq!(stack.pop(), None);

    stack.push("first");
    stack.push("second");
    stack.push("third");

    assert_eq!(stack.len(), 3);
    assert_eq!(stack.peek(), Some(&"third"));
    assert_eq!(stack.pop(), Some("third"));
    assert_eq!(stack.pop(), Some("second"));
    assert_eq!(stack.pop(), Some("first"));
    assert_eq!(stack.pop(), None);
    assert!(stack.is_empty());
}

#[test]
fn peek_mut_changes_only_the_top_value() {
    let mut stack = Stack::with_capacity(4);

    stack.push(String::from("draft"));
    stack.push(String::from("review"));

    stack.peek_mut().expect("top exists").push_str("-done");

    assert_eq!(stack.capacity(), 4);
    assert_eq!(stack.pop().as_deref(), Some("review-done"));
    assert_eq!(stack.pop().as_deref(), Some("draft"));
}

#[test]
fn clear_removes_values_but_keeps_capacity() {
    let mut stack = Stack::with_capacity(8);

    stack.push(1);
    stack.push(2);
    stack.clear();

    assert!(stack.is_empty());
    assert_eq!(stack.capacity(), 8);
    assert_eq!(stack.peek(), None);
}

#[test]
fn iter_reads_from_bottom_to_top_without_popping() {
    let mut stack = Stack::new();

    stack.push("base");
    stack.push("middle");
    stack.push("top");

    let values = stack.iter().copied().collect::<Vec<_>>();

    assert_eq!(values, vec!["base", "middle", "top"]);
    assert_eq!(stack.peek(), Some(&"top"));
    assert_eq!(stack.len(), 3);
}

#[test]
fn pop_transfers_ownership_and_clear_drops_remaining_values() {
    let drops = Rc::new(Cell::new(0));
    let mut stack = Stack::new();

    stack.push(DropCounter::new(Rc::clone(&drops)));
    stack.push(DropCounter::new(Rc::clone(&drops)));
    stack.push(DropCounter::new(Rc::clone(&drops)));

    let popped = stack.pop();
    assert_eq!(drops.get(), 0);

    drop(popped);
    assert_eq!(drops.get(), 1);

    stack.clear();
    assert_eq!(drops.get(), 3);
}

struct DropCounter {
    drops: Rc<Cell<usize>>,
}

impl DropCounter {
    fn new(drops: Rc<Cell<usize>>) -> Self {
        Self { drops }
    }
}

impl Drop for DropCounter {
    fn drop(&mut self) {
        self.drops.set(self.drops.get() + 1);
    }
}
