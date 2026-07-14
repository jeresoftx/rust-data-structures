use rust_data_structures::linked_list::LinkedList;
use rust_data_structures::stack::Stack;
use std::hint::black_box;
use std::time::{Duration, Instant};

const SIZE: usize = 50_000;

fn main() {
    println!("stack benchmark (manual, std::time::Instant)");
    println!("size: {SIZE}");
    println!("vector-backed push/pop: {:?}", bench_stack_push_pop());
    println!(
        "linked-list-backed push/pop: {:?}",
        bench_linked_list_push_pop()
    );
    println!("peek: {:?}", bench_peek());
}

fn bench_stack_push_pop() -> Duration {
    let start = Instant::now();
    let mut stack = Stack::new();

    for value in 0..SIZE {
        stack.push(black_box(value));
    }

    while let Some(value) = stack.pop() {
        black_box(value);
    }

    start.elapsed()
}

fn bench_linked_list_push_pop() -> Duration {
    let start = Instant::now();
    let mut list = LinkedList::new();

    for value in 0..SIZE {
        list.push_front(black_box(value));
    }

    while let Some(value) = list.pop_front() {
        black_box(value);
    }

    start.elapsed()
}

fn bench_peek() -> Duration {
    let mut stack = Stack::new();
    for value in 0..SIZE {
        stack.push(value);
    }

    let start = Instant::now();
    for _ in 0..SIZE {
        black_box(stack.peek());
    }

    start.elapsed()
}
