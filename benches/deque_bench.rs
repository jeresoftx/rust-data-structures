use rust_data_structures::deque::Deque;
use rust_data_structures::linked_list::LinkedList;
use rust_data_structures::vector::Vector;
use std::hint::black_box;
use std::time::{Duration, Instant};

const SIZE: usize = 30_000;

fn main() {
    println!("deque benchmark (manual, std::time::Instant)");
    println!("size: {SIZE}");
    println!("push/pop front: {:?}", bench_deque_front());
    println!("push/pop back: {:?}", bench_deque_back());
    println!("indexed access: {:?}", bench_deque_get());
    println!(
        "linked list front comparison: {:?}",
        bench_linked_list_front()
    );
    println!("vector front remove comparison: {:?}", bench_vector_front());
}

fn bench_deque_front() -> Duration {
    let start = Instant::now();
    let mut deque = Deque::new();

    for value in 0..SIZE {
        deque.push_front(black_box(value));
    }

    while let Some(value) = deque.pop_front() {
        black_box(value);
    }

    start.elapsed()
}

fn bench_deque_back() -> Duration {
    let start = Instant::now();
    let mut deque = Deque::new();

    for value in 0..SIZE {
        deque.push_back(black_box(value));
    }

    while let Some(value) = deque.pop_back() {
        black_box(value);
    }

    start.elapsed()
}

fn bench_deque_get() -> Duration {
    let start = Instant::now();
    let mut deque = Deque::new();

    for value in 0..SIZE {
        deque.push_back(value);
    }

    for index in 0..SIZE {
        black_box(deque.get(index));
    }

    start.elapsed()
}

fn bench_linked_list_front() -> Duration {
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

fn bench_vector_front() -> Duration {
    let start = Instant::now();
    let mut vector = Vector::new();

    for value in 0..SIZE {
        vector.push(black_box(value));
    }

    while !vector.is_empty() {
        black_box(vector.remove(0));
    }

    start.elapsed()
}
