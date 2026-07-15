use rust_data_structures::hashmap::HashMap;

fn main() {
    let mut cache = HashMap::new();

    cache.insert("language", "rust");
    cache.insert("course", "data-structures");

    println!("language: {:?}", cache.get(&"language"));
    println!("contiene course: {}", cache.contains_key(&"course"));
    println!("factor de carga: {:.2}", cache.load_factor());
}
