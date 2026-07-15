//! Skip list.
//!
//! Este modulo enseñara niveles probabilisticos, busqueda esperada logaritmica
//! y la relacion entre listas enlazadas y mapas ordenados.

const DEFAULT_MAX_LEVEL: usize = 16;
const DEFAULT_PROBABILITY: f64 = 0.5;
const DEFAULT_SEED: u64 = 0x5eed_2026_acad_0001;

/// Error al configurar una skip list.
///
/// Complejidad: construir o comparar este error cuesta O(1).
///
/// ```
/// use rust_data_structures::skip_list::{SkipList, SkipListError};
///
/// assert!(matches!(
///     SkipList::<i32>::with_seed(0, 0.5, 1),
///     Err(SkipListError::InvalidMaxLevel)
/// ));
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SkipListError {
    /// Una skip list necesita al menos un nivel.
    InvalidMaxLevel,
    /// La probabilidad debe estar entre 0 y 1, sin incluir extremos.
    InvalidProbability,
}

/// Skip list educativa con semilla reproducible.
///
/// Este tipo modela un conjunto ordenado: insertar un valor duplicado devuelve
/// `false` y no aumenta `len`.
///
/// Complejidad: busqueda, insercion y remocion son O(log n) esperadas.
///
/// ```
/// use rust_data_structures::skip_list::SkipList;
///
/// let mut list = SkipList::new();
/// list.insert(3);
/// list.insert(1);
/// list.insert(2);
///
/// assert!(list.contains(&2));
/// assert_eq!(list.iter().copied().collect::<Vec<_>>(), vec![1, 2, 3]);
/// ```
#[derive(Debug, Clone)]
pub struct SkipList<T> {
    nodes: Vec<Node<T>>,
    max_level: usize,
    probability: f64,
    current_level: usize,
    len: usize,
    rng_state: u64,
}

#[derive(Debug, Clone)]
struct Node<T> {
    value: Option<T>,
    forward: Vec<Option<usize>>,
}

/// Iterador ordenado sobre los valores de una skip list.
///
/// Complejidad: `next` cuesta O(1); consumirlo completo cuesta O(n).
///
/// ```
/// use rust_data_structures::skip_list::SkipList;
///
/// let mut list = SkipList::new();
/// list.insert(2);
/// list.insert(1);
///
/// assert_eq!(list.iter().copied().collect::<Vec<_>>(), vec![1, 2]);
/// ```
#[derive(Debug, Clone)]
pub struct Iter<'a, T> {
    list: &'a SkipList<T>,
    next: Option<usize>,
}

impl<T: Ord> SkipList<T> {
    /// Crea una skip list con parametros por defecto.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::skip_list::SkipList;
    ///
    /// let list = SkipList::<i32>::new();
    /// assert!(list.is_empty());
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::with_seed(DEFAULT_MAX_LEVEL, DEFAULT_PROBABILITY, DEFAULT_SEED)
            .expect("default skip list parameters are valid")
    }

    /// Crea una skip list con nivel maximo, probabilidad y semilla explicitos.
    ///
    /// La semilla hace que las pruebas y visualizaciones sean reproducibles.
    ///
    /// Complejidad: O(max_level).
    ///
    /// ```
    /// use rust_data_structures::skip_list::SkipList;
    ///
    /// let list = SkipList::<i32>::with_seed(8, 0.5, 42).unwrap();
    /// assert_eq!(list.max_level(), 8);
    /// ```
    pub fn with_seed(max_level: usize, probability: f64, seed: u64) -> Result<Self, SkipListError> {
        if max_level == 0 {
            return Err(SkipListError::InvalidMaxLevel);
        }
        if probability <= 0.0 || probability >= 1.0 {
            return Err(SkipListError::InvalidProbability);
        }

        Ok(Self {
            nodes: vec![Node::head(max_level)],
            max_level,
            probability,
            current_level: 1,
            len: 0,
            rng_state: seed,
        })
    }

    /// Devuelve el numero de valores almacenados.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::skip_list::SkipList;
    ///
    /// let mut list = SkipList::new();
    /// list.insert("a");
    /// assert_eq!(list.len(), 1);
    /// ```
    #[must_use]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Indica si la skip list esta vacia.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::skip_list::SkipList;
    ///
    /// let list = SkipList::<i32>::new();
    /// assert!(list.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Devuelve el nivel maximo configurado.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::skip_list::SkipList;
    ///
    /// let list = SkipList::<i32>::with_seed(6, 0.5, 1).unwrap();
    /// assert_eq!(list.max_level(), 6);
    /// ```
    #[must_use]
    pub fn max_level(&self) -> usize {
        self.max_level
    }

    /// Devuelve el nivel mas alto actualmente usado.
    ///
    /// El nivel minimo es 1, incluso cuando la lista esta vacia.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::skip_list::SkipList;
    ///
    /// let list = SkipList::<i32>::new();
    /// assert_eq!(list.current_level(), 1);
    /// ```
    #[must_use]
    pub fn current_level(&self) -> usize {
        self.current_level
    }

    /// Devuelve cuantos nodos tienen cada altura exacta.
    ///
    /// Es una metrica educativa para observar la distribucion probabilistica de
    /// niveles. La posicion `0` cuenta nodos de altura `1`.
    ///
    /// Complejidad: O(n + max_level).
    ///
    /// ```
    /// use rust_data_structures::skip_list::SkipList;
    ///
    /// let mut list = SkipList::with_seed(4, 0.5, 7).unwrap();
    /// list.insert(10);
    /// assert_eq!(list.level_histogram().iter().sum::<usize>(), 1);
    /// ```
    #[must_use]
    pub fn level_histogram(&self) -> Vec<usize> {
        let mut histogram = vec![0; self.max_level];

        for node in self.nodes.iter().skip(1) {
            if node.value.is_some() {
                histogram[node.forward.len() - 1] += 1;
            }
        }

        histogram
    }

    /// Busca un valor.
    ///
    /// Complejidad esperada: O(log n).
    ///
    /// ```
    /// use rust_data_structures::skip_list::SkipList;
    ///
    /// let mut list = SkipList::new();
    /// list.insert("rust");
    /// assert!(list.contains(&"rust"));
    /// ```
    #[must_use]
    pub fn contains(&self, value: &T) -> bool {
        self.find_index(value).is_some()
    }

    /// Inserta un valor y devuelve `true` si era nuevo.
    ///
    /// Complejidad esperada: O(log n).
    ///
    /// ```
    /// use rust_data_structures::skip_list::SkipList;
    ///
    /// let mut list = SkipList::new();
    /// assert!(list.insert(3));
    /// assert!(!list.insert(3));
    /// ```
    pub fn insert(&mut self, value: T) -> bool {
        let mut update = self.update_path(&value);
        if self.next_at_level_zero_matches(update[0], &value) {
            return false;
        }

        let node_level = self.random_level();
        if node_level > self.current_level {
            for slot in update.iter_mut().take(node_level).skip(self.current_level) {
                *slot = 0;
            }
            self.current_level = node_level;
        }

        let new_index = self.nodes.len();
        self.nodes.push(Node::new(value, node_level));

        for (level, previous_ref) in update.iter().enumerate().take(node_level) {
            let previous = *previous_ref;
            let next = self.nodes[previous].forward[level];
            self.nodes[new_index].forward[level] = next;
            self.nodes[previous].forward[level] = Some(new_index);
        }

        self.len += 1;
        true
    }

    /// Remueve un valor y lo devuelve si existia.
    ///
    /// Complejidad esperada: O(log n).
    ///
    /// ```
    /// use rust_data_structures::skip_list::SkipList;
    ///
    /// let mut list = SkipList::new();
    /// list.insert(3);
    /// assert_eq!(list.remove(&3), Some(3));
    /// assert_eq!(list.remove(&3), None);
    /// ```
    pub fn remove(&mut self, value: &T) -> Option<T> {
        let update = self.update_path(value);
        let target = self.nodes[update[0]].forward[0]?;

        if self.nodes[target].value.as_ref()? != value {
            return None;
        }

        for (level, previous_ref) in update.iter().enumerate().take(self.current_level) {
            let previous = *previous_ref;
            if self.nodes[previous].forward[level] == Some(target) {
                let replacement = self.nodes[target].forward.get(level).copied().flatten();
                self.nodes[previous].forward[level] = replacement;
            }
        }

        while self.current_level > 1 && self.nodes[0].forward[self.current_level - 1].is_none() {
            self.current_level -= 1;
        }

        self.len -= 1;
        self.nodes[target].value.take()
    }

    /// Elimina todos los valores y conserva la configuracion.
    ///
    /// Complejidad: O(n + max_level).
    ///
    /// ```
    /// use rust_data_structures::skip_list::SkipList;
    ///
    /// let mut list = SkipList::with_seed(4, 0.5, 1).unwrap();
    /// list.insert(1);
    /// list.clear();
    /// assert!(list.is_empty());
    /// assert_eq!(list.max_level(), 4);
    /// ```
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.nodes.push(Node::head(self.max_level));
        self.current_level = 1;
        self.len = 0;
    }

    /// Recorre los valores en orden ascendente.
    ///
    /// Complejidad: O(n).
    ///
    /// ```
    /// use rust_data_structures::skip_list::SkipList;
    ///
    /// let mut list = SkipList::new();
    /// list.insert(2);
    /// list.insert(1);
    /// assert_eq!(list.iter().copied().collect::<Vec<_>>(), vec![1, 2]);
    /// ```
    #[must_use]
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            list: self,
            next: self.nodes[0].forward[0],
        }
    }

    fn update_path(&self, value: &T) -> Vec<usize> {
        let mut update = vec![0; self.max_level];
        let mut current = 0;

        for level in (0..self.current_level).rev() {
            while let Some(next) = self.nodes[current].forward[level] {
                if self.nodes[next]
                    .value
                    .as_ref()
                    .expect("linked nodes always hold a value")
                    < value
                {
                    current = next;
                } else {
                    break;
                }
            }
            update[level] = current;
        }

        update
    }

    fn find_index(&self, value: &T) -> Option<usize> {
        let previous = self.update_path(value)[0];
        let candidate = self.nodes[previous].forward[0]?;

        (self.nodes[candidate].value.as_ref()? == value).then_some(candidate)
    }

    fn next_at_level_zero_matches(&self, previous: usize, value: &T) -> bool {
        let Some(candidate) = self.nodes[previous].forward[0] else {
            return false;
        };

        self.nodes[candidate]
            .value
            .as_ref()
            .is_some_and(|candidate_value| candidate_value == value)
    }

    fn random_level(&mut self) -> usize {
        let mut level = 1;
        while level < self.max_level && self.next_probability_sample() < self.probability {
            level += 1;
        }
        level
    }

    fn next_probability_sample(&mut self) -> f64 {
        self.rng_state = self
            .rng_state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        let mantissa = self.rng_state >> 11;
        mantissa as f64 / ((1_u64 << 53) as f64)
    }
}

impl<T: Ord> Default for SkipList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Node<T> {
    fn head(max_level: usize) -> Self {
        Self {
            value: None,
            forward: vec![None; max_level],
        }
    }

    fn new(value: T, level: usize) -> Self {
        Self {
            value: Some(value),
            forward: vec![None; level],
        }
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.next?;
        let node = &self.list.nodes[current];
        self.next = node.forward[0];
        node.value.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::SkipList;

    #[test]
    fn empty_list_starts_at_one_level() {
        let list = SkipList::<i32>::new();

        assert_eq!(list.current_level(), 1);
        assert_eq!(list.level_histogram().iter().sum::<usize>(), 0);
    }

    #[test]
    fn removing_last_tall_node_shrinks_current_level() {
        let mut list = SkipList::with_seed(4, 0.99, 1).unwrap();

        assert!(list.insert(10));
        assert!(list.current_level() > 1);

        assert_eq!(list.remove(&10), Some(10));
        assert_eq!(list.current_level(), 1);
    }
}
