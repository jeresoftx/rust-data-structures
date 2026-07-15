use rust_data_structures::btree::BTree;

fn main() {
    let mut index = BTree::new();

    index.insert("vector");
    index.insert("graph");
    index.insert("btree");

    println!("contiene graph: {}", index.contains(&"graph"));
    println!("capitulos ordenados:");

    for chapter in index.iter() {
        println!("- {chapter}");
    }
}
