use rust_data_structures::hashmap::HashMap;

fn main() {
    let mut counts = HashMap::new();

    for word in ["rust", "data", "rust", "systems", "data", "rust"] {
        let next_count = counts.get(&word).copied().unwrap_or(0) + 1;
        counts.insert(word, next_count);
    }

    for (word, count) in counts.iter() {
        println!("{word}: {count}");
    }
}
