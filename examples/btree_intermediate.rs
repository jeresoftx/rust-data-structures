use rust_data_structures::btree::BTree;

fn main() {
    let mut tree = BTree::with_min_degree(2).unwrap();

    for key in 1..=4 {
        tree.insert(key);
        println!(
            "insert {key}: len={}, height={}, root_keys={}",
            tree.len(),
            tree.height(),
            tree.root_key_count()
        );
    }

    println!(
        "orden final: {:?}",
        tree.iter().copied().collect::<Vec<_>>()
    );
}
