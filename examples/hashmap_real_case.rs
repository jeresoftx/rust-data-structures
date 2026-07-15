use rust_data_structures::hashmap::HashMap;

fn main() {
    let mut page_cache = HashMap::with_capacity(4);

    page_cache.insert("GET /courses/rust-data-structures", "200 cached");
    page_cache.insert("GET /courses/rust-algorithms", "200 cached");
    page_cache.insert("GET /health", "200 cached");

    println!(
        "respuesta cacheada: {:?}",
        page_cache.get(&"GET /courses/rust-data-structures")
    );

    page_cache.remove(&"GET /health");
    println!(
        "health sigue cacheado: {}",
        page_cache.contains_key(&"GET /health")
    );
}
