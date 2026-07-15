//! HashMap.
//!
//! Este modulo enseñara hashing, buckets, colisiones, factor de carga,
//! redimensionamiento y las compensaciones frente a estructuras ordenadas.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

const DEFAULT_BUCKETS: usize = 8;
const MAX_LOAD_FACTOR: f64 = 0.75;

#[derive(Debug, Clone)]
struct Entry<K, V> {
    key: K,
    value: V,
}

/// Tabla hash educativa con encadenamiento separado.
///
/// Complejidad: las operaciones de clave son O(1) esperadas y O(n) en el peor
/// caso si muchas claves caen en el mismo bucket.
///
/// ```
/// use rust_data_structures::hashmap::HashMap;
///
/// let mut map = HashMap::new();
/// map.insert("language", "rust");
///
/// assert_eq!(map.get(&"language"), Some(&"rust"));
/// assert!(map.contains_key(&"language"));
/// ```
#[derive(Debug, Clone)]
pub struct HashMap<K, V> {
    buckets: Vec<Vec<Entry<K, V>>>,
    len: usize,
}

/// Iterador sobre pares clave-valor.
///
/// Complejidad: `next` avanza sobre buckets y entradas; consumirlo completo
/// cuesta O(b + n), donde `b` es el numero de buckets.
///
/// ```
/// use rust_data_structures::hashmap::HashMap;
///
/// let mut map = HashMap::new();
/// map.insert("a", 1);
///
/// assert_eq!(map.iter().count(), 1);
/// ```
#[derive(Debug, Clone)]
pub struct Iter<'a, K, V> {
    map: &'a HashMap<K, V>,
    bucket_index: usize,
    entry_index: usize,
}

impl<K: Eq + Hash, V> HashMap<K, V> {
    /// Crea una tabla hash con capacidad inicial por defecto.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::hashmap::HashMap;
    ///
    /// let map = HashMap::<&str, i32>::new();
    /// assert!(map.is_empty());
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self::with_capacity(DEFAULT_BUCKETS)
    }

    /// Crea una tabla hash con al menos un bucket.
    ///
    /// Complejidad: O(b), donde `b` es el numero de buckets.
    ///
    /// ```
    /// use rust_data_structures::hashmap::HashMap;
    ///
    /// let map = HashMap::<&str, i32>::with_capacity(16);
    /// assert_eq!(map.capacity(), 16);
    /// ```
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            buckets: empty_buckets(capacity.max(1)),
            len: 0,
        }
    }

    /// Devuelve el numero de pares almacenados.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::hashmap::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// map.insert("a", 1);
    /// assert_eq!(map.len(), 1);
    /// ```
    #[must_use]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Indica si la tabla esta vacia.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::hashmap::HashMap;
    ///
    /// let map = HashMap::<&str, i32>::new();
    /// assert!(map.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Devuelve el numero de buckets.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::hashmap::HashMap;
    ///
    /// let map = HashMap::<&str, i32>::with_capacity(4);
    /// assert_eq!(map.capacity(), 4);
    /// ```
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.buckets.len()
    }

    /// Devuelve el factor de carga actual.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::hashmap::HashMap;
    ///
    /// let mut map = HashMap::with_capacity(4);
    /// map.insert("a", 1);
    /// assert_eq!(map.load_factor(), 0.25);
    /// ```
    #[must_use]
    pub fn load_factor(&self) -> f64 {
        self.len as f64 / self.capacity() as f64
    }

    /// Devuelve el tamano del bucket mas cargado.
    ///
    /// Es una metrica educativa para observar colisiones.
    ///
    /// Complejidad: O(b), donde `b` es el numero de buckets.
    ///
    /// ```
    /// use rust_data_structures::hashmap::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// map.insert("a", 1);
    /// assert!(map.max_bucket_len() >= 1);
    /// ```
    #[must_use]
    pub fn max_bucket_len(&self) -> usize {
        self.buckets
            .iter()
            .map(std::vec::Vec::len)
            .max()
            .unwrap_or(0)
    }

    /// Inserta o reemplaza un valor.
    ///
    /// Devuelve el valor anterior cuando la clave ya existia.
    ///
    /// Complejidad: O(1) esperada; O(n) en el peor caso por colisiones.
    ///
    /// ```
    /// use rust_data_structures::hashmap::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// assert_eq!(map.insert("lang", "rust"), None);
    /// assert_eq!(map.insert("lang", "Rust"), Some("rust"));
    /// ```
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.projected_load_factor() > MAX_LOAD_FACTOR {
            self.resize(self.capacity() * 2);
        }

        let bucket_index = self.bucket_index(&key);
        let bucket = &mut self.buckets[bucket_index];

        for entry in bucket {
            if entry.key == key {
                return Some(std::mem::replace(&mut entry.value, value));
            }
        }

        self.buckets[bucket_index].push(Entry { key, value });
        self.len += 1;
        None
    }

    /// Busca una clave.
    ///
    /// Complejidad: O(1) esperada; O(n) en el peor caso por colisiones.
    ///
    /// ```
    /// use rust_data_structures::hashmap::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// map.insert("a", 1);
    /// assert_eq!(map.get(&"a"), Some(&1));
    /// ```
    #[must_use]
    pub fn get(&self, key: &K) -> Option<&V> {
        self.buckets[self.bucket_index(key)]
            .iter()
            .find(|entry| entry.key == *key)
            .map(|entry| &entry.value)
    }

    /// Busca una clave y devuelve una referencia mutable a su valor.
    ///
    /// Complejidad: O(1) esperada; O(n) en el peor caso por colisiones.
    ///
    /// ```
    /// use rust_data_structures::hashmap::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// map.insert("a", 1);
    /// *map.get_mut(&"a").unwrap() += 1;
    /// assert_eq!(map.get(&"a"), Some(&2));
    /// ```
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let bucket_index = self.bucket_index(key);
        self.buckets[bucket_index]
            .iter_mut()
            .find(|entry| entry.key == *key)
            .map(|entry| &mut entry.value)
    }

    /// Indica si una clave existe.
    ///
    /// Complejidad: O(1) esperada; O(n) en el peor caso por colisiones.
    ///
    /// ```
    /// use rust_data_structures::hashmap::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// map.insert("a", 1);
    /// assert!(map.contains_key(&"a"));
    /// ```
    #[must_use]
    pub fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    /// Remueve una clave y devuelve su valor si existia.
    ///
    /// Complejidad: O(1) esperada; O(n) en el peor caso por colisiones.
    ///
    /// ```
    /// use rust_data_structures::hashmap::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// map.insert("a", 1);
    /// assert_eq!(map.remove(&"a"), Some(1));
    /// assert!(!map.contains_key(&"a"));
    /// ```
    pub fn remove(&mut self, key: &K) -> Option<V> {
        let bucket_index = self.bucket_index(key);
        let bucket = &mut self.buckets[bucket_index];
        let entry_index = bucket.iter().position(|entry| entry.key == *key)?;
        self.len -= 1;
        Some(bucket.swap_remove(entry_index).value)
    }

    /// Elimina todos los pares y conserva los buckets.
    ///
    /// Complejidad: O(b + n), donde `b` es el numero de buckets.
    ///
    /// ```
    /// use rust_data_structures::hashmap::HashMap;
    ///
    /// let mut map = HashMap::with_capacity(8);
    /// map.insert("a", 1);
    /// map.clear();
    /// assert!(map.is_empty());
    /// assert_eq!(map.capacity(), 8);
    /// ```
    pub fn clear(&mut self) {
        for bucket in &mut self.buckets {
            bucket.clear();
        }
        self.len = 0;
    }

    /// Itera sobre todos los pares.
    ///
    /// Complejidad: O(1) para crear el iterador y O(b + n) para consumirlo.
    ///
    /// ```
    /// use rust_data_structures::hashmap::HashMap;
    ///
    /// let mut map = HashMap::new();
    /// map.insert("a", 1);
    /// assert_eq!(map.iter().map(|(_, value)| *value).sum::<i32>(), 1);
    /// ```
    #[must_use]
    pub fn iter(&self) -> Iter<'_, K, V> {
        Iter {
            map: self,
            bucket_index: 0,
            entry_index: 0,
        }
    }

    fn projected_load_factor(&self) -> f64 {
        (self.len + 1) as f64 / self.capacity() as f64
    }

    fn resize(&mut self, new_capacity: usize) {
        let old_buckets = std::mem::replace(&mut self.buckets, empty_buckets(new_capacity));
        self.len = 0;

        for bucket in old_buckets {
            for entry in bucket {
                self.insert(entry.key, entry.value);
            }
        }
    }

    fn bucket_index(&self, key: &K) -> usize {
        (hash_key(key) as usize) % self.capacity()
    }
}

impl<K: Eq + Hash, V> Default for HashMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        while self.bucket_index < self.map.buckets.len() {
            if let Some(entry) = self.map.buckets[self.bucket_index].get(self.entry_index) {
                self.entry_index += 1;
                return Some((&entry.key, &entry.value));
            }

            self.bucket_index += 1;
            self.entry_index = 0;
        }

        None
    }
}

fn empty_buckets<K, V>(capacity: usize) -> Vec<Vec<Entry<K, V>>> {
    (0..capacity).map(|_| Vec::new()).collect()
}

fn hash_key<K: Hash>(key: &K) -> u64 {
    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod tests {
    use super::HashMap;

    #[test]
    fn clear_keeps_capacity_and_removes_values() {
        let mut map = HashMap::with_capacity(16);

        map.insert("a", 1);
        map.insert("b", 2);
        map.clear();

        assert_eq!(map.len(), 0);
        assert_eq!(map.capacity(), 16);
        assert_eq!(map.get(&"a"), None);
    }

    #[test]
    fn with_zero_capacity_still_creates_one_bucket() {
        let map = HashMap::<i32, i32>::with_capacity(0);

        assert_eq!(map.capacity(), 1);
        assert!(map.is_empty());
    }
}
