use rust_data_structures::bloom_filter::BloomFilter;

fn main() {
    let mut likely_cached = BloomFilter::with_estimated_items(500, 0.01).unwrap();

    for key in [
        "GET /courses/rust-data-structures",
        "GET /courses/rust-algorithms",
        "GET /manual/rfc-0001",
    ] {
        likely_cached.insert(&key);
    }

    for key in [
        "GET /manual/rfc-0001",
        "GET /private/admin",
        "GET /courses/rust-data-structures",
    ] {
        if likely_cached.might_contain(&key) {
            println!("consultar cache o base de datos: {key}");
        } else {
            println!("evitar lectura: {key} definitivamente no esta");
        }
    }
}
