use rust_data_structures::bloom_filter::BloomFilter;
use std::collections::HashSet;
use std::hint::black_box;
use std::time::{Duration, Instant};

const SIZE: usize = 20_000;

fn main() {
    println!("bloom filter benchmark (manual, std::time::Instant)");
    println!("size: {SIZE}");
    println!("insert: {:?}", bench_insert());
    println!("positive membership: {:?}", bench_positive_membership());
    println!("negative membership: {:?}", bench_negative_membership());
    println!("hashset exact membership: {:?}", bench_hashset_membership());
}

fn bench_insert() -> Duration {
    let start = Instant::now();
    let mut filter = BloomFilter::with_estimated_items(SIZE, 0.01).unwrap();
    for key in 0..SIZE {
        filter.insert(&black_box(key));
    }
    black_box(filter.set_bit_count());
    start.elapsed()
}

fn bench_positive_membership() -> Duration {
    let mut filter = BloomFilter::with_estimated_items(SIZE, 0.01).unwrap();
    for key in 0..SIZE {
        filter.insert(&key);
    }

    let start = Instant::now();
    for key in 0..SIZE {
        black_box(filter.might_contain(&key));
    }
    start.elapsed()
}

fn bench_negative_membership() -> Duration {
    let mut filter = BloomFilter::with_estimated_items(SIZE, 0.01).unwrap();
    for key in 0..SIZE {
        filter.insert(&key);
    }

    let start = Instant::now();
    for key in SIZE..(SIZE * 2) {
        black_box(filter.might_contain(&key));
    }
    start.elapsed()
}

fn bench_hashset_membership() -> Duration {
    let set: HashSet<usize> = (0..SIZE).collect();

    let start = Instant::now();
    for key in 0..SIZE {
        black_box(set.contains(&key));
    }
    start.elapsed()
}
