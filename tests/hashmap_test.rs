use rust_data_structures::hashmap::HashMap;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct CollidingKey(&'static str);

impl Hash for CollidingKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        1_u8.hash(state);
    }
}

#[test]
fn insert_and_lookup_track_membership_and_len() {
    let mut map = HashMap::new();

    assert!(map.is_empty());
    assert_eq!(map.len(), 0);

    assert_eq!(map.insert("language", "rust"), None);
    assert_eq!(map.insert("course", "data-structures"), None);

    assert_eq!(map.len(), 2);
    assert_eq!(map.get(&"language"), Some(&"rust"));
    assert_eq!(map.get(&"course"), Some(&"data-structures"));
    assert_eq!(map.get(&"missing"), None);
    assert!(map.contains_key(&"language"));
    assert!(!map.is_empty());
}

#[test]
fn inserting_existing_key_overwrites_value_without_growing_len() {
    let mut map = HashMap::new();

    assert_eq!(map.insert("chapter", 9), None);
    assert_eq!(map.insert("chapter", 10), Some(9));

    assert_eq!(map.len(), 1);
    assert_eq!(map.get(&"chapter"), Some(&10));
}

#[test]
fn separate_chaining_preserves_colliding_keys() {
    let mut map = HashMap::with_capacity(2);

    let first = CollidingKey("first");
    let second = CollidingKey("second");
    let third = CollidingKey("third");

    map.insert(first, 1);
    map.insert(second, 2);
    map.insert(third, 3);

    assert_eq!(map.get(&first), Some(&1));
    assert_eq!(map.get(&second), Some(&2));
    assert_eq!(map.get(&third), Some(&3));
    assert!(map.max_bucket_len() >= 3);
}

#[test]
fn remove_returns_value_and_keeps_other_bucket_entries() {
    let mut map = HashMap::with_capacity(2);

    let first = CollidingKey("first");
    let second = CollidingKey("second");

    map.insert(first, "a");
    map.insert(second, "b");

    assert_eq!(map.remove(&first), Some("a"));
    assert_eq!(map.remove(&first), None);
    assert_eq!(map.get(&second), Some(&"b"));
    assert_eq!(map.len(), 1);
}

#[test]
fn map_resizes_when_load_factor_gets_too_high() {
    let mut map = HashMap::with_capacity(4);
    let initial_capacity = map.capacity();

    for key in 0..10 {
        map.insert(key, key * 10);
    }

    assert!(map.capacity() > initial_capacity);
    assert!(map.load_factor() <= 0.75);
    for key in 0..10 {
        assert_eq!(map.get(&key), Some(&(key * 10)));
    }
}

#[test]
fn iter_visits_every_pair_once() {
    let mut map = HashMap::with_capacity(8);

    map.insert("b", 2);
    map.insert("a", 1);
    map.insert("c", 3);

    let mut pairs = map
        .iter()
        .map(|(key, value)| (*key, *value))
        .collect::<Vec<_>>();
    pairs.sort();

    assert_eq!(pairs, vec![("a", 1), ("b", 2), ("c", 3)]);
}

#[test]
fn get_mut_updates_value_in_place() {
    let mut map = HashMap::new();

    map.insert("hits", 1);
    *map.get_mut(&"hits").unwrap() += 1;

    assert_eq!(map.get(&"hits"), Some(&2));
}
