use rust_data_structures::bloom_filter::BloomFilter;

fn main() {
    let mut filter = BloomFilter::with_estimated_items(1_000, 0.02).unwrap();

    for id in 0..1_000 {
        filter.insert(&format!("visited:{id}"));
    }

    let false_positives = (10_000..11_000)
        .filter(|id| filter.might_contain(&format!("visited:{id}")))
        .count();
    let measured_rate = false_positives as f64 / 1_000.0;

    println!("falsos positivos medidos: {false_positives}");
    println!("tasa medida: {measured_rate:.4}");
    println!(
        "tasa estimada por formula: {:.4}",
        filter.estimated_false_positive_rate()
    );
}
