use rust_data_structures::trie::Trie;

fn main() {
    let mut trie = Trie::new();
    for word in ["app", "apple", "apply", "apt", "backend"] {
        trie.insert(word);
    }

    let suggestions = autocomplete(&trie, "ap", 3);

    assert_eq!(suggestions, vec!["app", "apple", "apply"]);
    println!("{suggestions:?}");
}

fn autocomplete(trie: &Trie, prefix: &str, limit: usize) -> Vec<String> {
    trie.words_with_prefix(prefix)
        .into_iter()
        .take(limit)
        .collect()
}
