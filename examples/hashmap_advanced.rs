use rust_data_structures::hashmap::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CollidingKey(&'static str);

impl Hash for CollidingKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        1_u8.hash(state);
    }
}

fn main() {
    let mut table = HashMap::with_capacity(2);

    for (key, value) in [
        (CollidingKey("parser"), 1),
        (CollidingKey("lexer"), 2),
        (CollidingKey("type-checker"), 3),
    ] {
        table.insert(key, value);
    }

    println!("len: {}", table.len());
    println!("bucket mas cargado: {}", table.max_bucket_len());
    println!("parser: {:?}", table.get(&CollidingKey("parser")));
}
