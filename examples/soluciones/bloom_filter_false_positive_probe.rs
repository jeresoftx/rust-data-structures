use rust_data_structures::bloom_filter::BloomFilter;

fn main() {
    let mut filter = BloomFilter::with_estimated_items(500, 0.02).unwrap();

    for id in 0..500 {
        filter.insert(&format!("url:{id}"));
    }

    let false_positives = (5_000..5_500)
        .filter(|id| filter.might_contain(&format!("url:{id}")))
        .count();

    assert!(false_positives < 40);
    println!("falsos positivos observados: {false_positives}");
}
