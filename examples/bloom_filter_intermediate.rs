use rust_data_structures::bloom_filter::BloomFilter;

fn main() {
    let mut filter = BloomFilter::with_estimated_items(100, 0.01).unwrap();

    for id in 0..40 {
        filter.insert(&format!("lesson:{id}"));
    }

    println!("bits: {}", filter.bit_count());
    println!("hashes: {}", filter.hash_count());
    println!("inserciones: {}", filter.inserted_count());
    println!(
        "falso positivo estimado: {:.4}",
        filter.estimated_false_positive_rate()
    );
}
