use rust_data_structures::heap::Heap;
use std::collections::BinaryHeap;
use std::hint::black_box;
use std::time::{Duration, Instant};

const SIZE: usize = 40_000;

fn main() {
    println!("heap benchmark (manual, std::time::Instant)");
    println!("size: {SIZE}");
    println!("custom heap push/pop: {:?}", bench_custom_heap_push_pop());
    println!("custom heapify: {:?}", bench_custom_heapify());
    println!("std BinaryHeap push/pop: {:?}", bench_std_binary_heap());
    println!("sorted vector priority queue: {:?}", bench_sorted_vector());
}

fn bench_custom_heap_push_pop() -> Duration {
    let start = Instant::now();
    let mut heap = Heap::new();

    for value in 0..SIZE {
        heap.push(black_box(value));
    }

    while let Some(value) = heap.pop() {
        black_box(value);
    }

    start.elapsed()
}

fn bench_custom_heapify() -> Duration {
    let values = (0..SIZE).rev().collect::<Vec<_>>();
    let start = Instant::now();
    let heap = Heap::from_vec(black_box(values));
    black_box(heap.peek());
    start.elapsed()
}

fn bench_std_binary_heap() -> Duration {
    let start = Instant::now();
    let mut heap = BinaryHeap::new();

    for value in 0..SIZE {
        heap.push(black_box(value));
    }

    while let Some(value) = heap.pop() {
        black_box(value);
    }

    start.elapsed()
}

fn bench_sorted_vector() -> Duration {
    let start = Instant::now();
    let mut values = Vec::new();

    for value in 0..SIZE {
        let index = values.binary_search(&value).unwrap_or_else(|index| index);
        values.insert(index, black_box(value));
    }

    while let Some(value) = values.pop() {
        black_box(value);
    }

    start.elapsed()
}
