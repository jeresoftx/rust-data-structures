use rust_data_structures::hashmap::HashMap;

fn main() {
    let mut cache = HashMap::new();

    cache.insert("user:1", "Ana");
    cache.insert("user:2", "Beatriz");

    assert_eq!(cache.remove(&"user:1"), Some("Ana"));
    assert_eq!(cache.remove(&"user:1"), None);
    assert_eq!(cache.get(&"user:2"), Some(&"Beatriz"));
    assert_eq!(cache.len(), 1);

    println!("cache actualizada despues de remove");
}
