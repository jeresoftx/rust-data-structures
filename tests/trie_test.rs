use rust_data_structures::trie::Trie;

#[test]
fn insert_and_lookup_distinguish_words_from_prefixes() {
    let mut trie = Trie::new();

    assert!(trie.is_empty());
    assert_eq!(trie.len(), 0);
    assert!(!trie.contains("car"));

    assert!(trie.insert("car"));
    assert!(trie.insert("cart"));
    assert!(!trie.insert("car"));

    assert_eq!(trie.len(), 2);
    assert!(trie.contains("car"));
    assert!(trie.contains("cart"));
    assert!(!trie.contains("ca"));
    assert!(trie.starts_with("ca"));
}

#[test]
fn empty_string_is_a_valid_terminal_word() {
    let mut trie = Trie::new();

    assert!(trie.insert(""));
    assert!(trie.insert("a"));
    assert!(!trie.insert(""));

    assert!(trie.contains(""));
    assert!(trie.starts_with(""));
    assert_eq!(trie.words_with_prefix(""), vec!["", "a"]);

    assert!(trie.remove(""));
    assert!(!trie.contains(""));
    assert!(trie.contains("a"));
}

#[test]
fn prefix_search_returns_sorted_words_from_shared_prefix() {
    let mut trie = Trie::new();

    trie.insert("app");
    trie.insert("apple");
    trie.insert("apply");
    trie.insert("bat");

    assert_eq!(trie.words_with_prefix("app"), vec!["app", "apple", "apply"]);
    assert_eq!(trie.words_with_prefix("missing"), Vec::<String>::new());
}

#[test]
fn remove_deletes_terminal_marker_without_breaking_siblings() {
    let mut trie = Trie::new();

    trie.insert("tea");
    trie.insert("team");
    trie.insert("tear");

    assert!(trie.remove("tea"));
    assert!(!trie.contains("tea"));
    assert!(trie.contains("team"));
    assert!(trie.contains("tear"));
    assert!(trie.starts_with("tea"));
    assert_eq!(trie.len(), 2);
}

#[test]
fn remove_prunes_dead_branches_and_reports_missing_words() {
    let mut trie = Trie::new();

    trie.insert("dog");
    assert!(trie.starts_with("do"));

    assert!(trie.remove("dog"));
    assert!(!trie.remove("dog"));
    assert!(!trie.contains("dog"));
    assert!(!trie.starts_with("do"));
    assert!(trie.is_empty());
}

#[test]
fn unicode_policy_uses_unicode_scalar_values_without_normalization() {
    let mut trie = Trie::new();

    trie.insert("nino");
    trie.insert("niño");
    trie.insert("ninja");
    trie.insert("🚀rust");
    trie.insert("é");

    assert!(trie.contains("niño"));
    assert!(trie.contains("🚀rust"));
    assert!(trie.starts_with("🚀"));
    assert_eq!(trie.words_with_prefix("ni"), vec!["ninja", "nino", "niño"]);
    assert!(!trie.contains("e\u{301}"));
}

#[test]
fn clear_removes_all_words_and_keeps_trie_reusable() {
    let mut trie = Trie::new();

    trie.insert("alpha");
    trie.insert("beta");
    trie.clear();

    assert!(trie.is_empty());
    assert_eq!(trie.len(), 0);
    assert!(!trie.contains("alpha"));

    assert!(trie.insert("gamma"));
    assert!(trie.contains("gamma"));
}
