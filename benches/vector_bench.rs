use rust_data_structures::vector::Vector;
use std::hint::black_box;
use std::time::{Duration, Instant};

const SIZE: usize = 20_000;

fn main() {
    println!("vector benchmark (manual, std::time::Instant)");
    println!("size: {SIZE}");
    println!("push growth: {:?}", bench_push_growth());
    println!("random access: {:?}", bench_random_access());
    println!("insert front: {:?}", bench_insert_front());
    println!("insert back: {:?}", bench_insert_back());
}

fn bench_push_growth() -> Duration {
    let start = Instant::now();
    let mut values = Vector::new();

    for value in 0..SIZE {
        values.push(black_box(value));
    }

    black_box(values.len());
    start.elapsed()
}

fn bench_random_access() -> Duration {
    let mut values = Vector::with_capacity(SIZE);
    for value in 0..SIZE {
        values.push(value);
    }

    let start = Instant::now();
    let mut sum = 0usize;

    for index in 0..SIZE {
        sum += *values.get(black_box(index)).expect("index inside vector");
    }

    black_box(sum);
    start.elapsed()
}

fn bench_insert_front() -> Duration {
    let start = Instant::now();
    let mut values = Vector::new();

    for value in 0..SIZE {
        values.insert(0, black_box(value)).expect("front is valid");
    }

    black_box(values.len());
    start.elapsed()
}

fn bench_insert_back() -> Duration {
    let start = Instant::now();
    let mut values = Vector::new();

    for value in 0..SIZE {
        values
            .insert(values.len(), black_box(value))
            .expect("len is valid insertion point");
    }

    black_box(values.len());
    start.elapsed()
}
