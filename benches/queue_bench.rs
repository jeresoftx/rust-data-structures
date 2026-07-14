use rust_data_structures::queue::Queue;
use rust_data_structures::vector::Vector;
use std::hint::black_box;
use std::time::{Duration, Instant};

const SIZE: usize = 30_000;

fn main() {
    println!("queue benchmark (manual, std::time::Instant)");
    println!("size: {SIZE}");
    println!("circular enqueue/dequeue: {:?}", bench_circular_queue());
    println!(
        "naive vector front remove: {:?}",
        bench_naive_vector_queue()
    );
    println!("wraparound reuse: {:?}", bench_wraparound_reuse());
}

fn bench_circular_queue() -> Duration {
    let start = Instant::now();
    let mut queue = Queue::new();

    for value in 0..SIZE {
        queue.enqueue(black_box(value));
    }

    while let Some(value) = queue.dequeue() {
        black_box(value);
    }

    start.elapsed()
}

fn bench_naive_vector_queue() -> Duration {
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

fn bench_wraparound_reuse() -> Duration {
    let start = Instant::now();
    let mut queue = Queue::with_capacity(128);

    for value in 0..SIZE {
        queue.enqueue(black_box(value));
        if value % 2 == 0 {
            black_box(queue.dequeue());
        }
    }

    while let Some(value) = queue.dequeue() {
        black_box(value);
    }

    start.elapsed()
}
