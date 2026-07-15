//! B-tree.
//!
//! Este modulo enseñara arboles multiway, busqueda ordenada, division de nodos
//! y localidad de memoria para indices.

use std::cmp::Ordering;

const DEFAULT_MIN_DEGREE: usize = 2;

/// Error al crear o configurar un B-tree.
///
/// Complejidad: construir o comparar este error cuesta O(1).
///
/// ```
/// use rust_data_structures::btree::{BTree, BTreeError};
///
/// assert!(matches!(
///     BTree::<i32>::with_min_degree(1),
///     Err(BTreeError::InvalidMinDegree)
/// ));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BTreeError {
    /// Un B-tree necesita grado minimo de al menos 2.
    InvalidMinDegree,
}

/// B-tree educativo de claves ordenadas.
///
/// Este tipo modela un conjunto: insertar una clave duplicada devuelve `false`
/// y no aumenta `len`.
///
/// Complejidad: busqueda, insercion e iteracion siguen los costos indicados en
/// cada metodo publico.
///
/// ```
/// use rust_data_structures::btree::BTree;
///
/// let mut tree = BTree::new();
/// tree.insert(3);
/// tree.insert(1);
/// tree.insert(2);
///
/// assert!(tree.contains(&2));
/// assert_eq!(tree.iter().copied().collect::<Vec<_>>(), vec![1, 2, 3]);
/// ```
#[derive(Debug, Clone)]
pub struct BTree<T> {
    root: Node<T>,
    min_degree: usize,
    len: usize,
}

#[derive(Debug, Clone)]
struct Node<T> {
    keys: Vec<T>,
    children: Vec<Node<T>>,
    leaf: bool,
}

/// Iterador in-order sobre las claves del B-tree.
///
/// Complejidad: `next` cuesta O(1); construirlo desde [`BTree::iter`] cuesta
/// O(n).
///
/// ```
/// use rust_data_structures::btree::BTree;
///
/// let mut tree = BTree::new();
/// tree.insert(2);
/// tree.insert(1);
///
/// assert_eq!(tree.iter().copied().collect::<Vec<_>>(), vec![1, 2]);
/// ```
#[derive(Debug, Clone)]
pub struct Iter<'a, T> {
    items: Vec<&'a T>,
    index: usize,
}

impl<T: Ord> BTree<T> {
    /// Crea un B-tree con grado minimo 2.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::btree::BTree;
    ///
    /// let tree = BTree::<i32>::new();
    /// assert!(tree.is_empty());
    /// assert_eq!(tree.min_degree(), 2);
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            root: Node::new_leaf(),
            min_degree: DEFAULT_MIN_DEGREE,
            len: 0,
        }
    }

    /// Crea un B-tree con grado minimo configurable.
    ///
    /// El grado minimo `t` define que cada nodo puede almacenar hasta
    /// `2t - 1` claves antes de dividirse.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::btree::BTree;
    ///
    /// let tree = BTree::<i32>::with_min_degree(3).unwrap();
    /// assert_eq!(tree.min_degree(), 3);
    /// ```
    pub fn with_min_degree(min_degree: usize) -> Result<Self, BTreeError> {
        if min_degree < 2 {
            return Err(BTreeError::InvalidMinDegree);
        }

        Ok(Self {
            root: Node::new_leaf(),
            min_degree,
            len: 0,
        })
    }

    /// Devuelve el numero de claves.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::btree::BTree;
    ///
    /// let mut tree = BTree::new();
    /// tree.insert("a");
    /// assert_eq!(tree.len(), 1);
    /// ```
    #[must_use]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Indica si el arbol esta vacio.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::btree::BTree;
    ///
    /// let tree = BTree::<i32>::new();
    /// assert!(tree.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Devuelve el grado minimo.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::btree::BTree;
    ///
    /// let tree = BTree::<i32>::with_min_degree(4).unwrap();
    /// assert_eq!(tree.min_degree(), 4);
    /// ```
    #[must_use]
    pub fn min_degree(&self) -> usize {
        self.min_degree
    }

    /// Devuelve la altura medida en niveles de nodos.
    ///
    /// Complejidad: O(h), donde `h` es la altura del arbol.
    ///
    /// ```
    /// use rust_data_structures::btree::BTree;
    ///
    /// let tree = BTree::<i32>::new();
    /// assert_eq!(tree.height(), 1);
    /// ```
    #[must_use]
    pub fn height(&self) -> usize {
        self.root.height()
    }

    /// Devuelve cuantas claves tiene la raiz.
    ///
    /// Este metodo existe para pruebas y visualizaciones educativas de split.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::btree::BTree;
    ///
    /// let mut tree = BTree::new();
    /// tree.insert(10);
    /// assert_eq!(tree.root_key_count(), 1);
    /// ```
    #[must_use]
    pub fn root_key_count(&self) -> usize {
        self.root.keys.len()
    }

    /// Busca una clave.
    ///
    /// Complejidad: O(log n) en altura del B-tree, con busqueda binaria dentro
    /// de cada nodo.
    ///
    /// ```
    /// use rust_data_structures::btree::BTree;
    ///
    /// let mut tree = BTree::new();
    /// tree.insert("clave");
    /// assert!(tree.contains(&"clave"));
    /// ```
    #[must_use]
    pub fn contains(&self, key: &T) -> bool {
        self.root.contains(key)
    }

    /// Inserta una clave y devuelve `true` si era nueva.
    ///
    /// Complejidad: O(log n) en altura del B-tree.
    ///
    /// ```
    /// use rust_data_structures::btree::BTree;
    ///
    /// let mut tree = BTree::new();
    /// assert!(tree.insert(7));
    /// assert!(!tree.insert(7));
    /// ```
    pub fn insert(&mut self, key: T) -> bool {
        if self.contains(&key) {
            return false;
        }

        if self.root.is_full(self.min_degree) {
            let old_root = std::mem::replace(&mut self.root, Node::new_leaf());
            let mut new_root = Node::new_internal();
            new_root.children.push(old_root);
            new_root.split_child(0, self.min_degree);
            self.root = new_root;
        }

        let inserted = self.root.insert_non_full(key, self.min_degree);
        if inserted {
            self.len += 1;
        }
        inserted
    }

    /// Recorre las claves en orden ascendente.
    ///
    /// El iterador materializa referencias para mantener el codigo del
    /// capitulo enfocado en la estructura, no en una pila manual de recorrido.
    ///
    /// Complejidad: O(n) para construir el iterador y O(1) por avance.
    ///
    /// ```
    /// use rust_data_structures::btree::BTree;
    ///
    /// let mut tree = BTree::new();
    /// tree.insert(2);
    /// tree.insert(1);
    /// assert_eq!(tree.iter().copied().collect::<Vec<_>>(), vec![1, 2]);
    /// ```
    #[must_use]
    pub fn iter(&self) -> Iter<'_, T> {
        let mut items = Vec::with_capacity(self.len);
        self.root.collect_refs(&mut items);
        Iter { items, index: 0 }
    }
}

impl<T: Ord> Default for BTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord> Node<T> {
    fn new_leaf() -> Self {
        Self {
            keys: Vec::new(),
            children: Vec::new(),
            leaf: true,
        }
    }

    fn new_internal() -> Self {
        Self {
            keys: Vec::new(),
            children: Vec::new(),
            leaf: false,
        }
    }

    fn is_full(&self, min_degree: usize) -> bool {
        self.keys.len() == (2 * min_degree) - 1
    }

    fn height(&self) -> usize {
        if self.leaf {
            1
        } else {
            1 + self.children[0].height()
        }
    }

    fn contains(&self, key: &T) -> bool {
        match self.keys.binary_search(key) {
            Ok(_) => true,
            Err(_) if self.leaf => false,
            Err(index) => self.children[index].contains(key),
        }
    }

    fn insert_non_full(&mut self, key: T, min_degree: usize) -> bool {
        if self.leaf {
            let index = self
                .keys
                .binary_search(&key)
                .expect_err("duplicates are checked before insertion");
            self.keys.insert(index, key);
            return true;
        }

        let mut child_index = match self.keys.binary_search(&key) {
            Ok(_) => return false,
            Err(index) => index,
        };

        if self.children[child_index].is_full(min_degree) {
            self.split_child(child_index, min_degree);

            match key.cmp(&self.keys[child_index]) {
                Ordering::Less => {}
                Ordering::Equal => return false,
                Ordering::Greater => child_index += 1,
            }
        }

        self.children[child_index].insert_non_full(key, min_degree)
    }

    fn split_child(&mut self, child_index: usize, min_degree: usize) {
        let (median, right) = {
            let child = &mut self.children[child_index];
            let right_keys = child.keys.split_off(min_degree);
            let median = child
                .keys
                .pop()
                .expect("full child has a median key before split");
            let right_children = if child.leaf {
                Vec::new()
            } else {
                child.children.split_off(min_degree)
            };

            (
                median,
                Node {
                    keys: right_keys,
                    children: right_children,
                    leaf: child.leaf,
                },
            )
        };

        self.keys.insert(child_index, median);
        self.children.insert(child_index + 1, right);
    }

    fn collect_refs<'a>(&'a self, items: &mut Vec<&'a T>) {
        if self.leaf {
            items.extend(self.keys.iter());
            return;
        }

        for index in 0..self.keys.len() {
            self.children[index].collect_refs(items);
            items.push(&self.keys[index]);
        }
        self.children[self.keys.len()].collect_refs(items);
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.items.get(self.index).copied();
        self.index += usize::from(item.is_some());
        item
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.items.len().saturating_sub(self.index);
        (remaining, Some(remaining))
    }
}

impl<T> ExactSizeIterator for Iter<'_, T> {}

#[cfg(test)]
mod tests {
    use super::BTree;

    #[test]
    fn empty_tree_height_is_one_root_level() {
        let tree = BTree::<i32>::new();

        assert_eq!(tree.height(), 1);
        assert_eq!(tree.root_key_count(), 0);
    }

    #[test]
    fn min_degree_controls_root_capacity_before_split() {
        let mut tree = BTree::with_min_degree(3).unwrap();

        for value in 1..=5 {
            tree.insert(value);
        }

        assert_eq!(tree.height(), 1);
        assert_eq!(tree.root_key_count(), 5);

        tree.insert(6);

        assert_eq!(tree.height(), 2);
        assert_eq!(tree.root_key_count(), 1);
    }
}
