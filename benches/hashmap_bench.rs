use rust_data_structures::hashmap::HashMap as AcademyHashMap;
use std::collections::HashMap as StdHashMap;
use std::hash::{Hash, Hasher};
use std::hint::black_box;
use std::time::{Duration, Instant};

const SIZE: i32 = 20_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CollidingKey(i32);

impl Hash for CollidingKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        1_u8.hash(state);
    }
}

fn main() {
    println!("hashmap benchmark (manual, std::time::Instant)");
    println!("size: {SIZE}");
    println!("custom insert normal: {:?}", bench_custom_insert_normal());
    println!("std insert normal: {:?}", bench_std_insert_normal());
    println!("custom lookup normal: {:?}", bench_custom_lookup_normal());
    println!(
        "custom insert high collision: {:?}",
        bench_custom_insert_collision()
    );
    println!(
        "custom lookup high collision: {:?}",
        bench_custom_lookup_collision()
    );
}

fn bench_custom_insert_normal() -> Duration {
    let start = Instant::now();
    let mut map = AcademyHashMap::with_capacity(1024);
    for key in 0..SIZE {
        black_box(map.insert(key, key * 2));
    }
    start.elapsed()
}

fn bench_std_insert_normal() -> Duration {
    let start = Instant::now();
    let mut map = StdHashMap::with_capacity(1024);
    for key in 0..SIZE {
        black_box(map.insert(key, key * 2));
    }
    start.elapsed()
}

fn bench_custom_lookup_normal() -> Duration {
    let mut map = AcademyHashMap::with_capacity(1024);
    for key in 0..SIZE {
        map.insert(key, key * 2);
    }

    let start = Instant::now();
    for key in 0..SIZE {
        black_box(map.get(&key));
    }
    start.elapsed()
}

fn bench_custom_insert_collision() -> Duration {
    let start = Instant::now();
    let mut map = AcademyHashMap::with_capacity(1024);
    for key in 0..SIZE {
        black_box(map.insert(CollidingKey(key), key * 2));
    }
    start.elapsed()
}

fn bench_custom_lookup_collision() -> Duration {
    let mut map = AcademyHashMap::with_capacity(1024);
    for key in 0..SIZE {
        map.insert(CollidingKey(key), key * 2);
    }

    let start = Instant::now();
    for key in 0..SIZE {
        black_box(map.get(&CollidingKey(key)));
    }
    start.elapsed()
}
