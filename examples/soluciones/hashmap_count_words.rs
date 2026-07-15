use rust_data_structures::hashmap::HashMap;

fn main() {
    let mut counts = HashMap::new();

    for word in ["hash", "bucket", "hash", "resize", "hash"] {
        let count = counts.get(&word).copied().unwrap_or(0);
        counts.insert(word, count + 1);
    }

    assert_eq!(counts.get(&"hash"), Some(&3));
    assert_eq!(counts.get(&"bucket"), Some(&1));
    assert_eq!(counts.get(&"missing"), None);

    println!("hash aparece {} veces", counts.get(&"hash").unwrap());
}
