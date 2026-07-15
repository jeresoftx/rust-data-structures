use rust_data_structures::bloom_filter::BloomFilter;

fn main() {
    let filter = BloomFilter::with_estimated_items(10_000, 0.01).unwrap();

    assert!(filter.bit_count() >= 95_000);
    assert!(filter.hash_count() >= 6);
    assert_eq!(filter.inserted_count(), 0);

    println!(
        "para 10000 elementos al 1%: {} bits y {} hashes",
        filter.bit_count(),
        filter.hash_count()
    );
}
