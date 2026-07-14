use rust_data_structures::trie::Trie;

fn main() {
    let mut trie = Trie::new();

    trie.insert("tea");
    trie.insert("team");
    trie.insert("tear");
    trie.remove("tea");

    println!("tea: {}", trie.contains("tea"));
    println!("team: {}", trie.contains("team"));
    println!("nodos internos: {}", trie.node_count());
}
