use rust_data_structures::btree::BTree as AcademyBTree;
use std::collections::BTreeSet;
use std::hint::black_box;
use std::time::{Duration, Instant};

const SIZE: i32 = 20_000;

fn main() {
    let ordered = ordered_values();
    let permuted = permuted_values();

    println!("btree benchmark (manual, std::time::Instant)");
    println!("size: {SIZE}");
    println!("custom ordered insert: {:?}", bench_custom_insert(&ordered));
    println!(
        "custom permuted insert: {:?}",
        bench_custom_insert(&permuted)
    );
    println!("std permuted insert: {:?}", bench_std_insert(&permuted));
    println!("custom search: {:?}", bench_custom_search(&permuted));
    println!("std search: {:?}", bench_std_search(&permuted));
}

fn bench_custom_insert(values: &[i32]) -> Duration {
    let start = Instant::now();
    let mut tree = AcademyBTree::with_min_degree(32).unwrap();
    for value in values {
        black_box(tree.insert(*value));
    }
    start.elapsed()
}

fn bench_std_insert(values: &[i32]) -> Duration {
    let start = Instant::now();
    let mut tree = BTreeSet::new();
    for value in values {
        black_box(tree.insert(*value));
    }
    start.elapsed()
}

fn bench_custom_search(values: &[i32]) -> Duration {
    let mut tree = AcademyBTree::with_min_degree(32).unwrap();
    for value in values {
        tree.insert(*value);
    }

    let start = Instant::now();
    for value in values {
        black_box(tree.contains(value));
    }
    start.elapsed()
}

fn bench_std_search(values: &[i32]) -> Duration {
    let tree = values.iter().copied().collect::<BTreeSet<_>>();

    let start = Instant::now();
    for value in values {
        black_box(tree.contains(value));
    }
    start.elapsed()
}

fn ordered_values() -> Vec<i32> {
    (0..SIZE).collect()
}

fn permuted_values() -> Vec<i32> {
    (0..SIZE).map(|value| (value * 37) % SIZE).collect()
}
