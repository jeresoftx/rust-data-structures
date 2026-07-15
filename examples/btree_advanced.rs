use rust_data_structures::btree::BTree;

fn main() {
    let mut index = BTree::with_min_degree(3).unwrap();
    let keys = [42, 7, 19, 73, 3, 11, 29, 61, 5, 17, 23, 31];

    for key in keys {
        index.insert(key);
    }

    let ordered = index.iter().copied().collect::<Vec<_>>();

    println!("grado minimo: {}", index.min_degree());
    println!("altura: {}", index.height());
    println!("ordenado: {ordered:?}");
    println!("busca 29: {}", index.contains(&29));
    println!("busca 100: {}", index.contains(&100));
}
