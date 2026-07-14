use rust_data_structures::trie::Trie;

fn main() {
    let mut dictionary = Trie::new();

    dictionary.insert("rust");
    dictionary.insert("runner");
    dictionary.insert("route");

    println!("contiene rust: {}", dictionary.contains("rust"));
    println!("prefijo ru: {}", dictionary.starts_with("ru"));
}
