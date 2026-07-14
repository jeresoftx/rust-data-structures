//! Deque.
//!
//! Este modulo enseñara colas de doble extremo, operaciones en ambos lados y
//! la representacion tipica con buffer circular.

/// Cola de doble extremo respaldada por un buffer circular.
///
/// El frente del deque se guarda en `head`; los elementos viven en orden logico
/// aunque puedan estar envueltos fisicamente.
///
/// ```
/// use rust_data_structures::deque::Deque;
///
/// let mut deque = Deque::new();
/// deque.push_back("middle");
/// deque.push_front("front");
///
/// assert_eq!(deque.front(), Some(&"front"));
/// assert_eq!(deque.pop_back(), Some("middle"));
/// ```
#[derive(Debug)]
pub struct Deque<T> {
    items: Box<[Option<T>]>,
    head: usize,
    len: usize,
}

/// Iterador inmutable sobre un [`Deque`], desde el frente hacia atras.
///
/// ```
/// use rust_data_structures::deque::Deque;
///
/// let mut deque = Deque::new();
/// deque.push_back(1);
/// deque.push_back(2);
///
/// assert_eq!(deque.iter().copied().collect::<Vec<_>>(), vec![1, 2]);
/// ```
#[derive(Debug, Clone)]
pub struct Iter<'a, T> {
    deque: &'a Deque<T>,
    offset: usize,
}

impl<T> Deque<T> {
    /// Crea un deque vacio.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::deque::Deque;
    ///
    /// let deque = Deque::<i32>::new();
    /// assert!(deque.is_empty());
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            items: allocate_slots(0),
            head: 0,
            len: 0,
        }
    }

    /// Crea un deque vacio con capacidad reservada.
    ///
    /// Complejidad: O(n), porque la version segura inicializa ranuras con
    /// `None`.
    ///
    /// ```
    /// use rust_data_structures::deque::Deque;
    ///
    /// let deque = Deque::<String>::with_capacity(4);
    /// assert_eq!(deque.capacity(), 4);
    /// ```
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            items: allocate_slots(capacity),
            head: 0,
            len: 0,
        }
    }

    /// Devuelve el numero de elementos.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::deque::Deque;
    ///
    /// let mut deque = Deque::new();
    /// deque.push_back("item");
    /// assert_eq!(deque.len(), 1);
    /// ```
    #[must_use]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Devuelve la capacidad reservada.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::deque::Deque;
    ///
    /// let deque = Deque::<u8>::with_capacity(3);
    /// assert_eq!(deque.capacity(), 3);
    /// ```
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.items.len()
    }

    /// Indica si el deque esta vacio.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::deque::Deque;
    ///
    /// let mut deque = Deque::new();
    /// assert!(deque.is_empty());
    /// deque.push_front(1);
    /// assert!(!deque.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Agrega `value` al frente.
    ///
    /// Complejidad: O(1) amortizado.
    ///
    /// ```
    /// use rust_data_structures::deque::Deque;
    ///
    /// let mut deque = Deque::new();
    /// deque.push_front("front");
    ///
    /// assert_eq!(deque.front(), Some(&"front"));
    /// ```
    pub fn push_front(&mut self, value: T) {
        if self.len == self.capacity() {
            self.grow();
        }

        self.head = if self.capacity() == 0 {
            0
        } else {
            (self.head + self.capacity() - 1) % self.capacity()
        };
        self.items[self.head] = Some(value);
        self.len += 1;
    }

    /// Agrega `value` al fondo.
    ///
    /// Complejidad: O(1) amortizado.
    ///
    /// ```
    /// use rust_data_structures::deque::Deque;
    ///
    /// let mut deque = Deque::new();
    /// deque.push_back("back");
    ///
    /// assert_eq!(deque.back(), Some(&"back"));
    /// ```
    pub fn push_back(&mut self, value: T) {
        if self.len == self.capacity() {
            self.grow();
        }

        let index = self.physical_index(self.len);
        self.items[index] = Some(value);
        self.len += 1;
    }

    /// Remueve y devuelve el frente, si existe.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::deque::Deque;
    ///
    /// let mut deque = Deque::new();
    /// deque.push_front(10);
    ///
    /// assert_eq!(deque.pop_front(), Some(10));
    /// assert_eq!(deque.pop_front(), None);
    /// ```
    pub fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let value = self.items[self.head].take();
        self.len -= 1;

        if self.len == 0 {
            self.head = 0;
        } else {
            self.head = (self.head + 1) % self.capacity();
        }

        value
    }

    /// Remueve y devuelve el fondo, si existe.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::deque::Deque;
    ///
    /// let mut deque = Deque::new();
    /// deque.push_back(10);
    ///
    /// assert_eq!(deque.pop_back(), Some(10));
    /// assert_eq!(deque.pop_back(), None);
    /// ```
    pub fn pop_back(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let index = self.physical_index(self.len - 1);
        let value = self.items[index].take();
        self.len -= 1;

        if self.len == 0 {
            self.head = 0;
        }

        value
    }

    /// Devuelve una referencia al frente.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::deque::Deque;
    ///
    /// let mut deque = Deque::new();
    /// deque.push_back("front");
    ///
    /// assert_eq!(deque.front(), Some(&"front"));
    /// ```
    #[must_use]
    pub fn front(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }

        self.items[self.head].as_ref()
    }

    /// Devuelve una referencia al fondo.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::deque::Deque;
    ///
    /// let mut deque = Deque::new();
    /// deque.push_back("front");
    /// deque.push_back("back");
    ///
    /// assert_eq!(deque.back(), Some(&"back"));
    /// ```
    #[must_use]
    pub fn back(&self) -> Option<&T> {
        self.get(self.len.checked_sub(1)?)
    }

    /// Devuelve una referencia por indice logico desde el frente.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::deque::Deque;
    ///
    /// let mut deque = Deque::new();
    /// deque.push_back("a");
    /// deque.push_back("b");
    ///
    /// assert_eq!(deque.get(1), Some(&"b"));
    /// assert_eq!(deque.get(2), None);
    /// ```
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }

        let physical = self.physical_index(index);
        self.items[physical].as_ref()
    }

    /// Remueve todos los elementos sin liberar capacidad.
    ///
    /// Complejidad: O(n).
    ///
    /// ```
    /// use rust_data_structures::deque::Deque;
    ///
    /// let mut deque = Deque::with_capacity(4);
    /// deque.push_back(1);
    /// deque.clear();
    ///
    /// assert!(deque.is_empty());
    /// assert_eq!(deque.capacity(), 4);
    /// ```
    pub fn clear(&mut self) {
        for offset in 0..self.len {
            let index = self.physical_index(offset);
            self.items[index] = None;
        }

        self.head = 0;
        self.len = 0;
    }

    /// Itera desde el frente hasta el fondo.
    ///
    /// Complejidad: O(1) para crear el iterador y O(n) para consumirlo completo.
    ///
    /// ```
    /// use rust_data_structures::deque::Deque;
    ///
    /// let mut deque = Deque::new();
    /// deque.push_back(1);
    /// deque.push_back(2);
    ///
    /// assert_eq!(deque.iter().copied().collect::<Vec<_>>(), vec![1, 2]);
    /// ```
    #[must_use]
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            deque: self,
            offset: 0,
        }
    }

    fn grow(&mut self) {
        let next_capacity = if self.capacity() == 0 {
            1
        } else {
            self.capacity() * 2
        };
        let mut next_items = allocate_slots(next_capacity);

        for offset in 0..self.len {
            let old_index = self.physical_index(offset);
            next_items[offset] = self.items[old_index].take();
        }

        self.items = next_items;
        self.head = 0;
    }

    fn physical_index(&self, offset: usize) -> usize {
        debug_assert!(self.capacity() > 0);
        (self.head + offset) % self.capacity()
    }
}

impl<T> Default for Deque<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset >= self.deque.len {
            return None;
        }

        let index = self.deque.physical_index(self.offset);
        self.offset += 1;

        self.deque.items[index].as_ref()
    }
}

fn allocate_slots<T>(capacity: usize) -> Box<[Option<T>]> {
    std::iter::repeat_with(|| None)
        .take(capacity)
        .collect::<Vec<_>>()
        .into_boxed_slice()
}

#[cfg(test)]
mod tests {
    use super::Deque;

    #[test]
    fn head_resets_after_deque_becomes_empty() {
        let mut deque = Deque::with_capacity(3);

        deque.push_back(1);
        deque.push_back(2);
        assert_eq!(deque.pop_front(), Some(1));
        assert_eq!(deque.pop_front(), Some(2));
        deque.push_back(3);

        assert_eq!(deque.iter().copied().collect::<Vec<_>>(), vec![3]);
        assert_eq!(deque.front(), Some(&3));
    }

    #[test]
    fn clear_is_idempotent_after_wraparound() {
        let mut deque = Deque::with_capacity(3);

        deque.push_back("a");
        deque.push_back("b");
        deque.push_back("c");
        deque.pop_front();
        deque.push_back("d");
        deque.clear();
        deque.clear();

        assert!(deque.is_empty());
        assert_eq!(deque.pop_back(), None);
    }
}
