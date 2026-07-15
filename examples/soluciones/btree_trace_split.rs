use rust_data_structures::btree::BTree;

fn main() {
    let mut tree = BTree::with_min_degree(2).unwrap();

    for value in 1..=4 {
        assert!(tree.insert(value));
    }

    assert_eq!(tree.height(), 2);
    assert_eq!(tree.root_key_count(), 1);
    assert_eq!(tree.iter().copied().collect::<Vec<_>>(), vec![1, 2, 3, 4]);

    println!("split de raiz confirmado");
}
