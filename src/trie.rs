//! Trie.
//!
//! Este modulo enseñara arboles de prefijos, marcadores terminales, busqueda
//! por prefijo y las compensaciones de memoria frente a mapas hash.

use std::collections::BTreeMap;

/// Arbol de prefijos basado en caracteres Unicode escalares de Rust.
///
/// La estructura no normaliza Unicode: dos representaciones canonicas distintas
/// se tratan como claves distintas.
///
/// ```
/// use rust_data_structures::trie::Trie;
///
/// let mut trie = Trie::new();
/// trie.insert("app");
/// trie.insert("apple");
///
/// assert!(trie.contains("app"));
/// assert!(trie.starts_with("ap"));
/// assert_eq!(trie.words_with_prefix("app"), vec!["app", "apple"]);
/// ```
#[derive(Debug, Clone, Default)]
pub struct Trie {
    root: Node,
    len: usize,
}

#[derive(Debug, Clone, Default)]
struct Node {
    is_terminal: bool,
    children: BTreeMap<char, Node>,
}

impl Trie {
    /// Crea un trie vacio.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::trie::Trie;
    ///
    /// let trie = Trie::new();
    /// assert!(trie.is_empty());
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Devuelve el numero de palabras almacenadas.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::trie::Trie;
    ///
    /// let mut trie = Trie::new();
    /// trie.insert("rust");
    /// assert_eq!(trie.len(), 1);
    /// ```
    #[must_use]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Indica si el trie no contiene palabras.
    ///
    /// Complejidad: O(1).
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Inserta una palabra y devuelve `true` si era nueva.
    ///
    /// Complejidad: O(m log a), donde `m` es el numero de `char` y `a` el
    /// numero de hijos por nodo.
    ///
    /// ```
    /// use rust_data_structures::trie::Trie;
    ///
    /// let mut trie = Trie::new();
    /// assert!(trie.insert("data"));
    /// assert!(!trie.insert("data"));
    /// ```
    pub fn insert(&mut self, word: &str) -> bool {
        let mut node = &mut self.root;

        for character in word.chars() {
            node = node.children.entry(character).or_default();
        }

        if node.is_terminal {
            return false;
        }

        node.is_terminal = true;
        self.len += 1;
        true
    }

    /// Indica si una palabra exacta existe.
    ///
    /// Complejidad: O(m log a).
    ///
    /// ```
    /// use rust_data_structures::trie::Trie;
    ///
    /// let mut trie = Trie::new();
    /// trie.insert("car");
    ///
    /// assert!(trie.contains("car"));
    /// assert!(!trie.contains("ca"));
    /// ```
    #[must_use]
    pub fn contains(&self, word: &str) -> bool {
        self.find_node(word).is_some_and(|node| node.is_terminal)
    }

    /// Indica si existe el prefijo, aunque no sea palabra terminal.
    ///
    /// Complejidad: O(m log a).
    ///
    /// ```
    /// use rust_data_structures::trie::Trie;
    ///
    /// let mut trie = Trie::new();
    /// trie.insert("cart");
    ///
    /// assert!(trie.starts_with("car"));
    /// ```
    #[must_use]
    pub fn starts_with(&self, prefix: &str) -> bool {
        self.find_node(prefix).is_some()
    }

    /// Remueve una palabra exacta y poda ramas que quedan sin uso.
    ///
    /// Complejidad: O(m log a).
    ///
    /// ```
    /// use rust_data_structures::trie::Trie;
    ///
    /// let mut trie = Trie::new();
    /// trie.insert("tea");
    /// trie.insert("team");
    ///
    /// assert!(trie.remove("tea"));
    /// assert!(trie.contains("team"));
    /// ```
    pub fn remove(&mut self, word: &str) -> bool {
        let characters = word.chars().collect::<Vec<_>>();
        let removed = remove_from_node(&mut self.root, &characters, 0).0;

        if removed {
            self.len -= 1;
        }

        removed
    }

    /// Devuelve palabras en orden lexicografico por `char` para un prefijo.
    ///
    /// Complejidad: O(m log a + r), donde `r` es el tamaño de la salida.
    ///
    /// ```
    /// use rust_data_structures::trie::Trie;
    ///
    /// let mut trie = Trie::new();
    /// trie.insert("app");
    /// trie.insert("apple");
    ///
    /// assert_eq!(trie.words_with_prefix("app"), vec!["app", "apple"]);
    /// ```
    #[must_use]
    pub fn words_with_prefix(&self, prefix: &str) -> Vec<String> {
        let Some(node) = self.find_node(prefix) else {
            return Vec::new();
        };

        let mut current = prefix.to_owned();
        let mut words = Vec::new();
        collect_words(node, &mut current, &mut words);
        words
    }

    /// Remueve todas las palabras.
    ///
    /// Complejidad: O(n), donde `n` es el numero de nodos.
    ///
    /// ```
    /// use rust_data_structures::trie::Trie;
    ///
    /// let mut trie = Trie::new();
    /// trie.insert("rust");
    /// trie.clear();
    ///
    /// assert!(trie.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.root = Node::default();
        self.len = 0;
    }

    /// Cuenta nodos de representacion, incluyendo la raiz.
    ///
    /// Complejidad: O(n).
    #[must_use]
    pub fn node_count(&self) -> usize {
        count_nodes(&self.root)
    }

    fn find_node(&self, prefix: &str) -> Option<&Node> {
        let mut node = &self.root;

        for character in prefix.chars() {
            node = node.children.get(&character)?;
        }

        Some(node)
    }
}

fn remove_from_node(node: &mut Node, characters: &[char], index: usize) -> (bool, bool) {
    if index == characters.len() {
        if !node.is_terminal {
            return (false, false);
        }

        node.is_terminal = false;
        return (true, node.children.is_empty());
    }

    let character = characters[index];
    let Some(child) = node.children.get_mut(&character) else {
        return (false, false);
    };

    let (removed, should_prune_child) = remove_from_node(child, characters, index + 1);

    if should_prune_child {
        node.children.remove(&character);
    }

    let should_prune_self = !node.is_terminal && node.children.is_empty();
    (removed, should_prune_self)
}

fn collect_words(node: &Node, current: &mut String, words: &mut Vec<String>) {
    if node.is_terminal {
        words.push(current.clone());
    }

    for (character, child) in &node.children {
        current.push(*character);
        collect_words(child, current, words);
        current.pop();
    }
}

fn count_nodes(node: &Node) -> usize {
    1 + node.children.values().map(count_nodes).sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::Trie;

    #[test]
    fn shared_prefixes_reuse_nodes() {
        let mut trie = Trie::new();

        trie.insert("car");
        trie.insert("cart");

        assert_eq!(trie.node_count(), 5);
    }

    #[test]
    fn removing_empty_string_does_not_prune_root() {
        let mut trie = Trie::new();

        trie.insert("");
        assert!(trie.remove(""));

        assert_eq!(trie.node_count(), 1);
        assert!(trie.starts_with(""));
    }
}
