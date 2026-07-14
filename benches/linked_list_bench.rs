use rust_data_structures::linked_list::LinkedList;
use rust_data_structures::vector::Vector;
use std::hint::black_box;
use std::time::{Duration, Instant};

const SIZE: usize = 20_000;

fn main() {
    println!("linked list benchmark (manual, std::time::Instant)");
    println!("size: {SIZE}");
    println!("push_front: {:?}", bench_push_front());
    println!("push_back: {:?}", bench_push_back());
    println!("pop_front: {:?}", bench_pop_front());
    println!("iterate linked list: {:?}", bench_linked_list_iteration());
    println!("iterate vector: {:?}", bench_vector_iteration());
}

fn bench_push_front() -> Duration {
    let start = Instant::now();
    let mut list = LinkedList::new();

    for value in 0..SIZE {
        list.push_front(black_box(value));
    }

    black_box(list.len());
    start.elapsed()
}

fn bench_push_back() -> Duration {
    let start = Instant::now();
    let mut list = LinkedList::new();

    for value in 0..SIZE {
        list.push_back(black_box(value));
    }

    black_box(list.len());
    start.elapsed()
}

fn bench_pop_front() -> Duration {
    let mut list = LinkedList::new();
    for value in 0..SIZE {
        list.push_back(value);
    }

    let start = Instant::now();
    while let Some(value) = list.pop_front() {
        black_box(value);
    }

    start.elapsed()
}

fn bench_linked_list_iteration() -> Duration {
    let mut list = LinkedList::new();
    for value in 0..SIZE {
        list.push_front(value);
    }

    let start = Instant::now();
    let sum: usize = list.iter().copied().map(black_box).sum();

    black_box(sum);
    start.elapsed()
}

fn bench_vector_iteration() -> Duration {
    let mut vector = Vector::with_capacity(SIZE);
    for value in 0..SIZE {
        vector.push(value);
    }

    let start = Instant::now();
    let sum: usize = vector.iter().copied().map(black_box).sum();

    black_box(sum);
    start.elapsed()
}
