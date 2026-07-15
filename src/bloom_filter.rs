//! Filtro de Bloom.
//!
//! Este modulo enseñara pertenencia probabilistica, falsos positivos, funciones
//! hash multiples y dimensionamiento.

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// Error al crear un filtro de Bloom.
///
/// Complejidad: construir o comparar este error cuesta O(1).
///
/// ```
/// use rust_data_structures::bloom_filter::{BloomFilter, BloomFilterError};
///
/// assert!(matches!(
///     BloomFilter::new(0, 3),
///     Err(BloomFilterError::InvalidBitCount)
/// ));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BloomFilterError {
    /// El numero de bits debe ser mayor que cero.
    InvalidBitCount,
    /// El numero de funciones hash debe ser mayor que cero.
    InvalidHashCount,
    /// La estimacion de elementos debe ser mayor que cero.
    InvalidExpectedItems,
    /// La tasa de falso positivo debe estar entre 0 y 1, sin incluir extremos.
    InvalidFalsePositiveRate,
}

/// Filtro de Bloom para pertenencia probabilistica.
///
/// Complejidad: insertar y consultar cuesta O(k), donde `k` es el numero de
/// hashes configurados.
///
/// ```
/// use rust_data_structures::bloom_filter::BloomFilter;
///
/// let mut filter = BloomFilter::new(128, 3).unwrap();
/// filter.insert(&"rust");
///
/// assert!(filter.might_contain(&"rust"));
/// ```
#[derive(Debug, Clone)]
pub struct BloomFilter {
    bits: Vec<bool>,
    hash_count: usize,
    inserted_count: usize,
}

impl BloomFilter {
    /// Crea un filtro con numero de bits y hashes especifico.
    ///
    /// Complejidad: O(m), donde `m` es el numero de bits inicializados.
    ///
    /// ```
    /// use rust_data_structures::bloom_filter::BloomFilter;
    ///
    /// let filter = BloomFilter::new(64, 3).unwrap();
    /// assert_eq!(filter.bit_count(), 64);
    /// assert_eq!(filter.hash_count(), 3);
    /// ```
    pub fn new(bit_count: usize, hash_count: usize) -> Result<Self, BloomFilterError> {
        if bit_count == 0 {
            return Err(BloomFilterError::InvalidBitCount);
        }
        if hash_count == 0 {
            return Err(BloomFilterError::InvalidHashCount);
        }

        Ok(Self {
            bits: vec![false; bit_count],
            hash_count,
            inserted_count: 0,
        })
    }

    /// Crea un filtro a partir de elementos esperados y tasa objetivo.
    ///
    /// Complejidad: O(m), donde `m` es el numero de bits calculados.
    ///
    /// ```
    /// use rust_data_structures::bloom_filter::BloomFilter;
    ///
    /// let filter = BloomFilter::with_estimated_items(100, 0.01).unwrap();
    /// assert!(filter.bit_count() > 0);
    /// assert!(filter.hash_count() > 0);
    /// ```
    pub fn with_estimated_items(
        expected_items: usize,
        false_positive_rate: f64,
    ) -> Result<Self, BloomFilterError> {
        if expected_items == 0 {
            return Err(BloomFilterError::InvalidExpectedItems);
        }
        if false_positive_rate <= 0.0 || false_positive_rate >= 1.0 {
            return Err(BloomFilterError::InvalidFalsePositiveRate);
        }

        let expected = expected_items as f64;
        let bit_count = (-(expected * false_positive_rate.ln()) / std::f64::consts::LN_2.powi(2))
            .ceil() as usize;
        let hash_count = ((bit_count as f64 / expected) * std::f64::consts::LN_2).ceil() as usize;

        Self::new(bit_count.max(1), hash_count.max(1))
    }

    /// Inserta un valor.
    ///
    /// Complejidad: O(k), donde `k` es el numero de hashes.
    ///
    /// ```
    /// use rust_data_structures::bloom_filter::BloomFilter;
    ///
    /// let mut filter = BloomFilter::new(64, 3).unwrap();
    /// filter.insert(&"key");
    ///
    /// assert!(filter.might_contain(&"key"));
    /// assert_eq!(filter.inserted_count(), 1);
    /// ```
    pub fn insert<T: Hash>(&mut self, value: &T) {
        let indexes = self.indexes(value);
        for index in indexes {
            self.bits[index] = true;
        }
        self.inserted_count += 1;
    }

    /// Devuelve `false` si el valor definitivamente no esta.
    ///
    /// Devuelve `true` si el valor podria estar presente.
    ///
    /// Complejidad: O(k).
    ///
    /// ```
    /// use rust_data_structures::bloom_filter::BloomFilter;
    ///
    /// let filter = BloomFilter::new(64, 3).unwrap();
    ///
    /// assert!(!filter.might_contain(&"missing"));
    /// ```
    #[must_use]
    pub fn might_contain<T: Hash>(&self, value: &T) -> bool {
        self.indexes(value)
            .into_iter()
            .all(|index| self.bits[index])
    }

    /// Limpia todos los bits y reinicia el contador educativo.
    ///
    /// Complejidad: O(m), donde `m` es el numero de bits.
    ///
    /// ```
    /// use rust_data_structures::bloom_filter::BloomFilter;
    ///
    /// let mut filter = BloomFilter::new(64, 3).unwrap();
    /// filter.insert(&"key");
    /// filter.clear();
    ///
    /// assert_eq!(filter.inserted_count(), 0);
    /// assert_eq!(filter.set_bit_count(), 0);
    /// ```
    pub fn clear(&mut self) {
        self.bits.fill(false);
        self.inserted_count = 0;
    }

    /// Devuelve el numero de bits.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::bloom_filter::BloomFilter;
    ///
    /// let filter = BloomFilter::new(32, 2).unwrap();
    /// assert_eq!(filter.bit_count(), 32);
    /// ```
    #[must_use]
    pub fn bit_count(&self) -> usize {
        self.bits.len()
    }

    /// Devuelve el numero de hashes simulados.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::bloom_filter::BloomFilter;
    ///
    /// let filter = BloomFilter::new(32, 2).unwrap();
    /// assert_eq!(filter.hash_count(), 2);
    /// ```
    #[must_use]
    pub fn hash_count(&self) -> usize {
        self.hash_count
    }

    /// Devuelve cuantas inserciones se han realizado.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::bloom_filter::BloomFilter;
    ///
    /// let mut filter = BloomFilter::new(32, 2).unwrap();
    /// filter.insert(&"a");
    /// assert_eq!(filter.inserted_count(), 1);
    /// ```
    #[must_use]
    pub fn inserted_count(&self) -> usize {
        self.inserted_count
    }

    /// Devuelve cuantos bits estan encendidos.
    ///
    /// Complejidad: O(m), donde `m` es el numero de bits.
    ///
    /// ```
    /// use rust_data_structures::bloom_filter::BloomFilter;
    ///
    /// let mut filter = BloomFilter::new(32, 2).unwrap();
    /// filter.insert(&"a");
    /// assert!(filter.set_bit_count() > 0);
    /// ```
    #[must_use]
    pub fn set_bit_count(&self) -> usize {
        self.bits.iter().filter(|bit| **bit).count()
    }

    /// Estima la tasa de falso positivo con el estado actual.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::bloom_filter::BloomFilter;
    ///
    /// let filter = BloomFilter::new(32, 2).unwrap();
    /// assert_eq!(filter.estimated_false_positive_rate(), 0.0);
    /// ```
    #[must_use]
    pub fn estimated_false_positive_rate(&self) -> f64 {
        if self.inserted_count == 0 {
            return 0.0;
        }

        let k = self.hash_count as f64;
        let n = self.inserted_count as f64;
        let m = self.bit_count() as f64;

        (1.0 - (-(k * n) / m).exp()).powf(k)
    }

    fn indexes<T: Hash>(&self, value: &T) -> Vec<usize> {
        let hash_a = hash_with_seed(value, 0x9e37_79b9_7f4a_7c15);
        let hash_b = hash_with_seed(value, 0xc2b2_ae3d_27d4_eb4f) | 1;
        let bit_count = self.bit_count() as u64;

        (0..self.hash_count)
            .map(|index| hash_a.wrapping_add((index as u64).wrapping_mul(hash_b)) % bit_count)
            .map(|index| index as usize)
            .collect()
    }
}

fn hash_with_seed<T: Hash>(value: &T, seed: u64) -> u64 {
    let mut hasher = DefaultHasher::new();
    seed.hash(&mut hasher);
    value.hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod tests {
    use super::BloomFilter;

    #[test]
    fn empty_filter_has_zero_estimated_false_positive_rate() {
        let filter = BloomFilter::new(64, 3).unwrap();

        assert_eq!(filter.estimated_false_positive_rate(), 0.0);
    }

    #[test]
    fn insertion_sets_at_most_hash_count_bits() {
        let mut filter = BloomFilter::new(64, 4).unwrap();

        filter.insert(&"rust");

        assert!(filter.set_bit_count() <= filter.hash_count());
        assert!(filter.set_bit_count() > 0);
    }
}
