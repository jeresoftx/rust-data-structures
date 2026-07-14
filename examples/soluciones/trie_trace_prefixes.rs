use rust_data_structures::trie::Trie;

fn main() {
    let mut trie = Trie::new();
    trie.insert("car");
    trie.insert("cart");
    trie.insert("cat");

    assert!(trie.contains("car"));
    assert!(!trie.contains("ca"));
    assert!(trie.starts_with("ca"));
    assert_eq!(trie.words_with_prefix("car"), vec!["car", "cart"]);

    println!("{:?}", trie.words_with_prefix("ca"));
}
