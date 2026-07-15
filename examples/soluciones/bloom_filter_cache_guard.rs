use rust_data_structures::bloom_filter::BloomFilter;

fn main() {
    let mut guard = BloomFilter::with_estimated_items(100, 0.01).unwrap();

    for key in ["user:1", "user:2", "course:rust-data-structures"] {
        guard.insert(&key);
    }

    assert!(guard.might_contain(&"user:1"));
    assert!(!guard.might_contain(&"course:missing"));

    println!("guardia de cache listo");
}
