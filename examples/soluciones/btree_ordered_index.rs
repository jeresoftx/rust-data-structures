use rust_data_structures::btree::BTree;

fn main() {
    let mut index = BTree::new();

    for key in [30, 10, 20, 40] {
        index.insert(key);
    }

    let ordered = index.iter().copied().collect::<Vec<_>>();

    assert_eq!(ordered, vec![10, 20, 30, 40]);
    assert!(index.contains(&30));
    assert!(!index.contains(&99));

    println!("indice ordenado: {ordered:?}");
}
