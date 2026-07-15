use rust_data_structures::skip_list::SkipList;
use std::collections::BTreeSet;
use std::hint::black_box;
use std::time::{Duration, Instant};

const SIZE: i32 = 20_000;

fn main() {
    println!("skip list benchmark (manual, std::time::Instant)");
    println!("size: {SIZE}");
    println!("ordered insert: {:?}", bench_ordered_insert());
    println!("permuted insert: {:?}", bench_permuted_insert());
    println!("search: {:?}", bench_search());
    println!("iteration: {:?}", bench_iteration());
    println!("std btreeset search: {:?}", bench_btreeset_search());
}

fn bench_ordered_insert() -> Duration {
    let start = Instant::now();
    let mut list = SkipList::with_seed(20, 0.5, 1).unwrap();
    for value in 0..SIZE {
        black_box(list.insert(value));
    }
    start.elapsed()
}

fn bench_permuted_insert() -> Duration {
    let start = Instant::now();
    let mut list = SkipList::with_seed(20, 0.5, 1).unwrap();
    for value in permuted_values() {
        black_box(list.insert(value));
    }
    start.elapsed()
}

fn bench_search() -> Duration {
    let mut list = SkipList::with_seed(20, 0.5, 1).unwrap();
    for value in permuted_values() {
        list.insert(value);
    }

    let start = Instant::now();
    for value in 0..SIZE {
        black_box(list.contains(&value));
    }
    start.elapsed()
}

fn bench_iteration() -> Duration {
    let mut list = SkipList::with_seed(20, 0.5, 1).unwrap();
    for value in permuted_values() {
        list.insert(value);
    }

    let start = Instant::now();
    for value in list.iter() {
        black_box(value);
    }
    start.elapsed()
}

fn bench_btreeset_search() -> Duration {
    let set = permuted_values().into_iter().collect::<BTreeSet<_>>();

    let start = Instant::now();
    for value in 0..SIZE {
        black_box(set.contains(&value));
    }
    start.elapsed()
}

fn permuted_values() -> Vec<i32> {
    (0..SIZE).map(|value| (value * 37) % SIZE).collect()
}
