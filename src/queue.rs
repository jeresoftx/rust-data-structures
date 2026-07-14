//! Cola.
//!
//! Este modulo enseñara la abstraccion FIFO, el costo de remover al frente de
//! un vector y la motivacion de buffers circulares.

/// Cola FIFO respaldada por un buffer circular.
///
/// El frente de la cola se guarda en `head`; los elementos viven en orden
/// logico aunque puedan estar envueltos fisicamente dentro del arreglo.
///
/// ```
/// use rust_data_structures::queue::Queue;
///
/// let mut queue = Queue::new();
/// queue.enqueue("first");
/// queue.enqueue("second");
///
/// assert_eq!(queue.front(), Some(&"first"));
/// assert_eq!(queue.dequeue(), Some("first"));
/// assert_eq!(queue.dequeue(), Some("second"));
/// ```
#[derive(Debug)]
pub struct Queue<T> {
    items: Box<[Option<T>]>,
    head: usize,
    len: usize,
}

/// Iterador inmutable sobre una [`Queue`], desde el frente hacia atras.
///
/// ```
/// use rust_data_structures::queue::Queue;
///
/// let mut queue = Queue::new();
/// queue.enqueue(1);
/// queue.enqueue(2);
///
/// assert_eq!(queue.iter().copied().collect::<Vec<_>>(), vec![1, 2]);
/// ```
#[derive(Debug, Clone)]
pub struct Iter<'a, T> {
    queue: &'a Queue<T>,
    offset: usize,
}

impl<T> Queue<T> {
    /// Crea una cola vacia.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::queue::Queue;
    ///
    /// let queue = Queue::<i32>::new();
    /// assert!(queue.is_empty());
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            items: allocate_slots(0),
            head: 0,
            len: 0,
        }
    }

    /// Crea una cola vacia con capacidad reservada.
    ///
    /// Complejidad: O(n), porque la version segura inicializa ranuras con
    /// `None`.
    ///
    /// ```
    /// use rust_data_structures::queue::Queue;
    ///
    /// let queue = Queue::<String>::with_capacity(4);
    /// assert_eq!(queue.capacity(), 4);
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
    /// use rust_data_structures::queue::Queue;
    ///
    /// let mut queue = Queue::new();
    /// queue.enqueue("job");
    /// assert_eq!(queue.len(), 1);
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
    /// use rust_data_structures::queue::Queue;
    ///
    /// let queue = Queue::<u8>::with_capacity(3);
    /// assert_eq!(queue.capacity(), 3);
    /// ```
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.items.len()
    }

    /// Indica si la cola esta vacia.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::queue::Queue;
    ///
    /// let mut queue = Queue::new();
    /// assert!(queue.is_empty());
    /// queue.enqueue(1);
    /// assert!(!queue.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Agrega `value` al final de la cola.
    ///
    /// Complejidad: O(1) amortizado.
    ///
    /// ```
    /// use rust_data_structures::queue::Queue;
    ///
    /// let mut queue = Queue::new();
    /// queue.enqueue("first");
    /// queue.enqueue("second");
    ///
    /// assert_eq!(queue.back(), Some(&"second"));
    /// ```
    pub fn enqueue(&mut self, value: T) {
        if self.len == self.capacity() {
            self.grow();
        }

        let index = self.physical_index(self.len);
        self.items[index] = Some(value);
        self.len += 1;
    }

    /// Remueve y devuelve el frente de la cola, si existe.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::queue::Queue;
    ///
    /// let mut queue = Queue::new();
    /// queue.enqueue(10);
    ///
    /// assert_eq!(queue.dequeue(), Some(10));
    /// assert_eq!(queue.dequeue(), None);
    /// ```
    pub fn dequeue(&mut self) -> Option<T> {
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

    /// Devuelve una referencia al frente de la cola.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::queue::Queue;
    ///
    /// let mut queue = Queue::new();
    /// queue.enqueue("front");
    ///
    /// assert_eq!(queue.front(), Some(&"front"));
    /// ```
    #[must_use]
    pub fn front(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }

        self.items[self.head].as_ref()
    }

    /// Devuelve una referencia al ultimo elemento de la cola.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::queue::Queue;
    ///
    /// let mut queue = Queue::new();
    /// queue.enqueue("front");
    /// queue.enqueue("back");
    ///
    /// assert_eq!(queue.back(), Some(&"back"));
    /// ```
    #[must_use]
    pub fn back(&self) -> Option<&T> {
        if self.is_empty() {
            return None;
        }

        let index = self.physical_index(self.len - 1);
        self.items[index].as_ref()
    }

    /// Remueve todos los elementos sin liberar capacidad.
    ///
    /// Complejidad: O(n).
    ///
    /// ```
    /// use rust_data_structures::queue::Queue;
    ///
    /// let mut queue = Queue::with_capacity(4);
    /// queue.enqueue(1);
    /// queue.clear();
    ///
    /// assert!(queue.is_empty());
    /// assert_eq!(queue.capacity(), 4);
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
    /// use rust_data_structures::queue::Queue;
    ///
    /// let mut queue = Queue::new();
    /// queue.enqueue(1);
    /// queue.enqueue(2);
    ///
    /// assert_eq!(queue.iter().copied().collect::<Vec<_>>(), vec![1, 2]);
    /// ```
    #[must_use]
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            queue: self,
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

impl<T> Default for Queue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.offset >= self.queue.len {
            return None;
        }

        let index = self.queue.physical_index(self.offset);
        self.offset += 1;

        self.queue.items[index].as_ref()
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
    use super::Queue;

    #[test]
    fn head_resets_after_queue_becomes_empty() {
        let mut queue = Queue::with_capacity(3);

        queue.enqueue(1);
        queue.enqueue(2);
        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        queue.enqueue(3);

        assert_eq!(queue.iter().copied().collect::<Vec<_>>(), vec![3]);
        assert_eq!(queue.front(), Some(&3));
    }

    #[test]
    fn clear_is_idempotent_after_wraparound() {
        let mut queue = Queue::with_capacity(3);

        queue.enqueue("a");
        queue.enqueue("b");
        queue.enqueue("c");
        queue.dequeue();
        queue.enqueue("d");
        queue.clear();
        queue.clear();

        assert!(queue.is_empty());
        assert_eq!(queue.dequeue(), None);
    }
}
