use rust_data_structures::vector::Vector;

#[test]
fn push_and_pop_preserve_lifo_order_at_the_back() {
    let mut values = Vector::new();

    assert!(values.is_empty());
    assert_eq!(values.len(), 0);
    assert_eq!(values.capacity(), 0);

    values.push("a");
    values.push("b");

    assert_eq!(values.len(), 2);
    assert!(values.capacity() >= 2);
    assert_eq!(values.pop(), Some("b"));
    assert_eq!(values.pop(), Some("a"));
    assert_eq!(values.pop(), None);
    assert!(values.is_empty());
}

#[test]
fn get_and_get_mut_access_existing_indexes_only() {
    let mut values = Vector::with_capacity(4);

    values.push(10);
    values.push(20);

    assert_eq!(values.capacity(), 4);
    assert_eq!(values.get(0), Some(&10));
    assert_eq!(values.get(1), Some(&20));
    assert_eq!(values.get(2), None);

    *values.get_mut(1).expect("index 1 exists") = 25;

    assert_eq!(values.get(1), Some(&25));
}

#[test]
fn insert_and_remove_shift_elements_without_losing_order() {
    let mut values = Vector::new();

    values.push(1);
    values.push(3);
    values
        .insert(1, 2)
        .expect("index 1 is a valid insertion point");
    values.insert(3, 4).expect("len is a valid insertion point");

    assert_eq!(values.len(), 4);
    assert_eq!(values.get(0), Some(&1));
    assert_eq!(values.get(1), Some(&2));
    assert_eq!(values.get(2), Some(&3));
    assert_eq!(values.get(3), Some(&4));
    assert!(values.insert(5, 99).is_err());

    assert_eq!(values.remove(1), Some(2));
    assert_eq!(values.remove(2), Some(4));
    assert_eq!(values.remove(2), None);
    assert_eq!(values.get(0), Some(&1));
    assert_eq!(values.get(1), Some(&3));
}

#[test]
fn iter_and_clear_expose_values_without_leaking_capacity() {
    let mut values = Vector::with_capacity(8);

    values.push("vector");
    values.push("heap");
    values.push("trie");

    let collected = values.iter().copied().collect::<Vec<_>>();
    assert_eq!(collected, vec!["vector", "heap", "trie"]);

    values.clear();

    assert!(values.is_empty());
    assert_eq!(values.capacity(), 8);
    assert_eq!(values.iter().next(), None);
}
