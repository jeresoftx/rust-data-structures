use rust_data_structures::btree::{BTree, BTreeError};

#[test]
fn insertion_and_search_track_membership_and_len() {
    let mut tree = BTree::new();

    assert!(tree.is_empty());
    assert_eq!(tree.len(), 0);

    assert!(tree.insert(30));
    assert!(tree.insert(10));
    assert!(tree.insert(20));

    assert_eq!(tree.len(), 3);
    assert!(tree.contains(&10));
    assert!(tree.contains(&20));
    assert!(tree.contains(&30));
    assert!(!tree.contains(&40));
    assert!(!tree.is_empty());
}

#[test]
fn duplicate_insertions_keep_original_key_set() {
    let mut tree = BTree::new();

    assert!(tree.insert("rust"));
    assert!(!tree.insert("rust"));

    assert_eq!(tree.len(), 1);
    assert_eq!(tree.iter().copied().collect::<Vec<_>>(), vec!["rust"]);
}

#[test]
fn root_splits_when_capacity_is_exceeded() {
    let mut tree = BTree::with_min_degree(2).unwrap();

    for value in 1..=4 {
        assert!(tree.insert(value));
    }

    assert_eq!(tree.len(), 4);
    assert_eq!(tree.height(), 2);
    assert_eq!(tree.root_key_count(), 1);
    assert_eq!(tree.iter().copied().collect::<Vec<_>>(), vec![1, 2, 3, 4]);
}

#[test]
fn sorted_iteration_survives_multiple_splits() {
    let mut tree = BTree::with_min_degree(3).unwrap();
    let values = [8, 3, 12, 1, 6, 10, 14, 4, 7, 13, 2, 5, 9, 11];

    for value in values {
        assert!(tree.insert(value));
    }

    assert_eq!(
        tree.iter().copied().collect::<Vec<_>>(),
        vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]
    );
}

#[test]
fn invalid_min_degree_is_rejected() {
    assert!(matches!(
        BTree::<i32>::with_min_degree(1),
        Err(BTreeError::InvalidMinDegree)
    ));
}
