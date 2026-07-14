use rust_data_structures::trie::Trie;

fn main() {
    let mut autocomplete = Trie::new();

    autocomplete.insert("app");
    autocomplete.insert("apple");
    autocomplete.insert("apply");
    autocomplete.insert("backend");

    println!("{:?}", autocomplete.words_with_prefix("app"));
}
