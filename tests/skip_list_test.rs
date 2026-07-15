use rust_data_structures::skip_list::{SkipList, SkipListError};

#[test]
fn insert_and_search_track_membership_and_len() {
    let mut list = SkipList::new();

    assert!(list.is_empty());
    assert_eq!(list.len(), 0);

    assert!(list.insert(30));
    assert!(list.insert(10));
    assert!(list.insert(20));

    assert_eq!(list.len(), 3);
    assert!(list.contains(&10));
    assert!(list.contains(&20));
    assert!(list.contains(&30));
    assert!(!list.contains(&40));
    assert!(!list.is_empty());
}

#[test]
fn iteration_returns_sorted_values() {
    let mut list = SkipList::with_seed(8, 0.5, 17).unwrap();

    for value in [8, 3, 12, 1, 6, 10, 14, 4, 7, 13, 2, 5, 9, 11] {
        assert!(list.insert(value));
    }

    assert_eq!(
        list.iter().copied().collect::<Vec<_>>(),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]
    );
}

#[test]
fn duplicate_insertions_keep_original_set() {
    let mut list = SkipList::new();

    assert!(list.insert("rust"));
    assert!(!list.insert("rust"));

    assert_eq!(list.len(), 1);
    assert_eq!(list.iter().copied().collect::<Vec<_>>(), vec!["rust"]);
}

#[test]
fn remove_unlinks_values_without_breaking_order() {
    let mut list = SkipList::with_seed(8, 0.5, 99).unwrap();

    for value in [10, 20, 30, 40, 50] {
        assert!(list.insert(value));
    }

    assert_eq!(list.remove(&30), Some(30));
    assert_eq!(list.remove(&10), Some(10));
    assert_eq!(list.remove(&99), None);

    assert_eq!(list.len(), 3);
    assert!(!list.contains(&30));
    assert_eq!(list.iter().copied().collect::<Vec<_>>(), vec![20, 40, 50]);
}

#[test]
fn seeded_level_generation_is_reproducible() {
    let mut first = SkipList::with_seed(12, 0.5, 2026).unwrap();
    let mut second = SkipList::with_seed(12, 0.5, 2026).unwrap();

    for value in 0..64 {
        assert!(first.insert(value));
        assert!(second.insert(value));
    }

    assert_eq!(first.level_histogram(), second.level_histogram());
    assert_eq!(first.current_level(), second.current_level());
    assert!(first.current_level() <= first.max_level());
}

#[test]
fn clear_resets_values_and_levels() {
    let mut list = SkipList::with_seed(6, 0.5, 7).unwrap();

    for value in 0..10 {
        assert!(list.insert(value));
    }

    list.clear();

    assert!(list.is_empty());
    assert_eq!(list.len(), 0);
    assert_eq!(list.current_level(), 1);
    assert_eq!(list.iter().next(), None);
}

#[test]
fn configuration_rejects_invalid_parameters() {
    assert!(matches!(
        SkipList::<i32>::with_seed(0, 0.5, 1),
        Err(SkipListError::InvalidMaxLevel)
    ));
    assert!(matches!(
        SkipList::<i32>::with_seed(4, 0.0, 1),
        Err(SkipListError::InvalidProbability)
    ));
    assert!(matches!(
        SkipList::<i32>::with_seed(4, 1.0, 1),
        Err(SkipListError::InvalidProbability)
    ));
}
