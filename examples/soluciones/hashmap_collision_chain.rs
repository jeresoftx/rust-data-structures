use rust_data_structures::hashmap::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CollidingKey(&'static str);

impl Hash for CollidingKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        7_u8.hash(state);
    }
}

fn main() {
    let mut map = HashMap::with_capacity(2);

    let a = CollidingKey("a");
    let b = CollidingKey("b");
    let c = CollidingKey("c");

    map.insert(a, 1);
    map.insert(b, 2);
    map.insert(c, 3);

    assert_eq!(map.get(&a), Some(&1));
    assert_eq!(map.get(&b), Some(&2));
    assert_eq!(map.get(&c), Some(&3));
    assert!(map.max_bucket_len() >= 3);

    println!("colisiones preservadas en cadena");
}
