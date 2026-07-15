use rust_data_structures::btree::BTree;

fn main() {
    let mut tree = BTree::new();

    assert!(tree.insert("course-001"));
    assert!(!tree.insert("course-001"));

    assert_eq!(tree.len(), 1);
    assert_eq!(tree.iter().copied().collect::<Vec<_>>(), vec!["course-001"]);

    println!("duplicados rechazados sin cambiar len");
}
