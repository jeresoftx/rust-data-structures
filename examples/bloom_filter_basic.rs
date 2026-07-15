use rust_data_structures::bloom_filter::BloomFilter;

fn main() {
    let mut filter = BloomFilter::new(128, 3).unwrap();

    filter.insert(&"rust");
    filter.insert(&"data-structures");

    println!("rust podria estar: {}", filter.might_contain(&"rust"));
    println!(
        "python definitivamente no esta: {}",
        !filter.might_contain(&"python")
    );
    println!("bits encendidos: {}", filter.set_bit_count());
}
