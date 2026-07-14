use rust_data_structures::trie::Trie;

fn main() {
    let mut trie = Trie::new();
    trie.insert("é");
    trie.insert("e\u{301}");
    trie.insert("🚀rust");

    assert!(trie.contains("é"));
    assert!(trie.contains("e\u{301}"));
    assert!(trie.starts_with("🚀"));
    assert_eq!(trie.words_with_prefix("🚀"), vec!["🚀rust"]);

    println!("{:?}", trie.words_with_prefix(""));
}
