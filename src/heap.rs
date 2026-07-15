//! Heap.
//!
//! Este modulo enseñara monticulos binarios, propiedad de heap, `sift up`,
//! `sift down`, `heapify` y colas de prioridad.

/// Monticulo binario maximo.
///
/// La raiz contiene el valor de mayor prioridad segun [`Ord`].
///
/// Complejidad: insercion y remocion cuestan O(log n); leer la raiz cuesta
/// O(1).
///
/// ```
/// use rust_data_structures::heap::Heap;
///
/// let mut heap = Heap::new();
/// heap.push(2);
/// heap.push(5);
///
/// assert_eq!(heap.peek(), Some(&5));
/// assert_eq!(heap.pop(), Some(5));
/// ```
#[derive(Debug, Clone)]
pub struct Heap<T> {
    items: Vec<T>,
}

/// Monticulo binario minimo.
///
/// La raiz contiene el valor de menor prioridad segun [`Ord`].
///
/// Complejidad: insercion y remocion cuestan O(log n); leer la raiz cuesta
/// O(1).
///
/// ```
/// use rust_data_structures::heap::MinHeap;
///
/// let mut heap = MinHeap::new();
/// heap.push(2);
/// heap.push(5);
///
/// assert_eq!(heap.peek(), Some(&2));
/// ```
#[derive(Debug, Clone)]
pub struct MinHeap<T> {
    items: Vec<T>,
}

impl<T: Ord> Heap<T> {
    /// Crea un heap maximo vacio.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::heap::Heap;
    ///
    /// let heap = Heap::<i32>::new();
    /// assert!(heap.is_empty());
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Crea un heap maximo vacio con capacidad reservada.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::heap::Heap;
    ///
    /// let heap = Heap::<i32>::with_capacity(8);
    /// assert!(heap.capacity() >= 8);
    /// ```
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            items: Vec::with_capacity(capacity),
        }
    }

    /// Construye un heap maximo desde valores existentes usando heapify.
    ///
    /// Complejidad: O(n).
    ///
    /// ```
    /// use rust_data_structures::heap::Heap;
    ///
    /// let heap = Heap::from_vec(vec![1, 5, 3]);
    /// assert_eq!(heap.peek(), Some(&5));
    /// ```
    #[must_use]
    pub fn from_vec(items: Vec<T>) -> Self {
        let mut heap = Self { items };
        heap.heapify();
        heap
    }

    /// Devuelve el numero de elementos.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::heap::Heap;
    ///
    /// let mut heap = Heap::new();
    /// heap.push(1);
    /// assert_eq!(heap.len(), 1);
    /// ```
    #[must_use]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Devuelve la capacidad reservada.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::heap::Heap;
    ///
    /// let heap = Heap::<i32>::with_capacity(4);
    /// assert!(heap.capacity() >= 4);
    /// ```
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.items.capacity()
    }

    /// Indica si el heap esta vacio.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::heap::Heap;
    ///
    /// let heap = Heap::<i32>::new();
    /// assert!(heap.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Inserta un valor y restaura la propiedad de heap.
    ///
    /// Complejidad: O(log n).
    ///
    /// ```
    /// use rust_data_structures::heap::Heap;
    ///
    /// let mut heap = Heap::new();
    /// heap.push(4);
    /// heap.push(9);
    ///
    /// assert_eq!(heap.peek(), Some(&9));
    /// ```
    pub fn push(&mut self, value: T) {
        self.items.push(value);
        self.sift_up(self.len() - 1);
    }

    /// Remueve y devuelve el maximo, si existe.
    ///
    /// Complejidad: O(log n).
    ///
    /// ```
    /// use rust_data_structures::heap::Heap;
    ///
    /// let mut heap = Heap::from_vec(vec![1, 5, 3]);
    /// assert_eq!(heap.pop(), Some(5));
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        let last = self.items.pop()?;

        if self.items.is_empty() {
            return Some(last);
        }

        let root = std::mem::replace(&mut self.items[0], last);
        self.sift_down(0);
        Some(root)
    }

    /// Lee el maximo sin removerlo.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::heap::Heap;
    ///
    /// let heap = Heap::from_vec(vec![1, 3, 2]);
    /// assert_eq!(heap.peek(), Some(&3));
    /// ```
    #[must_use]
    pub fn peek(&self) -> Option<&T> {
        self.items.first()
    }

    /// Remueve todos los elementos sin liberar capacidad.
    ///
    /// Complejidad: O(n).
    ///
    /// ```
    /// use rust_data_structures::heap::Heap;
    ///
    /// let mut heap = Heap::with_capacity(4);
    /// heap.push(1);
    /// heap.clear();
    /// assert!(heap.is_empty());
    /// assert!(heap.capacity() >= 4);
    /// ```
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Itera en orden de arreglo, nivel por nivel.
    ///
    /// Este iterador no devuelve valores ordenados; muestra la representacion
    /// interna del heap.
    ///
    /// Complejidad: O(1) para crear el iterador y O(n) para consumirlo completo.
    ///
    /// ```
    /// use rust_data_structures::heap::Heap;
    ///
    /// let heap = Heap::from_vec(vec![1, 2, 3]);
    /// assert_eq!(heap.iter_level_order().count(), 3);
    /// ```
    pub fn iter_level_order(&self) -> std::slice::Iter<'_, T> {
        self.items.iter()
    }

    fn heapify(&mut self) {
        if self.len() < 2 {
            return;
        }

        let last_parent = (self.len() / 2).saturating_sub(1);
        for index in (0..=last_parent).rev() {
            self.sift_down(index);
        }
    }

    fn sift_up(&mut self, mut index: usize) {
        while index > 0 {
            let parent = parent_index(index);
            if self.items[parent] >= self.items[index] {
                break;
            }

            self.items.swap(parent, index);
            index = parent;
        }
    }

    fn sift_down(&mut self, mut index: usize) {
        loop {
            let left = left_child_index(index);
            let right = right_child_index(index);
            let mut largest = index;

            if left < self.len() && self.items[left] > self.items[largest] {
                largest = left;
            }

            if right < self.len() && self.items[right] > self.items[largest] {
                largest = right;
            }

            if largest == index {
                break;
            }

            self.items.swap(index, largest);
            index = largest;
        }
    }
}

impl<T: Ord> Default for Heap<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord> MinHeap<T> {
    /// Crea un heap minimo vacio.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::heap::MinHeap;
    ///
    /// let heap = MinHeap::<i32>::new();
    /// assert!(heap.is_empty());
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Crea un heap minimo vacio con capacidad reservada.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::heap::MinHeap;
    ///
    /// let heap = MinHeap::<i32>::with_capacity(8);
    /// assert!(heap.capacity() >= 8);
    /// ```
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            items: Vec::with_capacity(capacity),
        }
    }

    /// Construye un heap minimo desde valores existentes usando heapify.
    ///
    /// Complejidad: O(n).
    ///
    /// ```
    /// use rust_data_structures::heap::MinHeap;
    ///
    /// let heap = MinHeap::from_vec(vec![3, 1, 2]);
    /// assert_eq!(heap.peek(), Some(&1));
    /// ```
    #[must_use]
    pub fn from_vec(items: Vec<T>) -> Self {
        let mut heap = Self { items };
        heap.heapify();
        heap
    }

    /// Devuelve el numero de elementos.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::heap::MinHeap;
    ///
    /// let mut heap = MinHeap::new();
    /// heap.push(1);
    /// assert_eq!(heap.len(), 1);
    /// ```
    #[must_use]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Devuelve la capacidad reservada.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::heap::MinHeap;
    ///
    /// let heap = MinHeap::<i32>::with_capacity(4);
    /// assert!(heap.capacity() >= 4);
    /// ```
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.items.capacity()
    }

    /// Indica si el heap esta vacio.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::heap::MinHeap;
    ///
    /// let heap = MinHeap::<i32>::new();
    /// assert!(heap.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Inserta un valor y restaura la propiedad de heap minimo.
    ///
    /// Complejidad: O(log n).
    ///
    /// ```
    /// use rust_data_structures::heap::MinHeap;
    ///
    /// let mut heap = MinHeap::new();
    /// heap.push(4);
    /// heap.push(1);
    /// assert_eq!(heap.peek(), Some(&1));
    /// ```
    pub fn push(&mut self, value: T) {
        self.items.push(value);
        self.sift_up(self.len() - 1);
    }

    /// Remueve y devuelve el minimo, si existe.
    ///
    /// Complejidad: O(log n).
    ///
    /// ```
    /// use rust_data_structures::heap::MinHeap;
    ///
    /// let mut heap = MinHeap::from_vec(vec![3, 1, 2]);
    /// assert_eq!(heap.pop(), Some(1));
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        let last = self.items.pop()?;

        if self.items.is_empty() {
            return Some(last);
        }

        let root = std::mem::replace(&mut self.items[0], last);
        self.sift_down(0);
        Some(root)
    }

    /// Lee el minimo sin removerlo.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::heap::MinHeap;
    ///
    /// let heap = MinHeap::from_vec(vec![3, 1, 2]);
    /// assert_eq!(heap.peek(), Some(&1));
    /// ```
    #[must_use]
    pub fn peek(&self) -> Option<&T> {
        self.items.first()
    }

    /// Remueve todos los elementos sin liberar capacidad.
    ///
    /// Complejidad: O(n).
    ///
    /// ```
    /// use rust_data_structures::heap::MinHeap;
    ///
    /// let mut heap = MinHeap::with_capacity(4);
    /// heap.push(1);
    /// heap.clear();
    /// assert!(heap.is_empty());
    /// assert!(heap.capacity() >= 4);
    /// ```
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Itera en orden de arreglo, nivel por nivel.
    ///
    /// Complejidad: O(1) para crear el iterador y O(n) para consumirlo completo.
    ///
    /// ```
    /// use rust_data_structures::heap::MinHeap;
    ///
    /// let heap = MinHeap::from_vec(vec![3, 1, 2]);
    /// assert_eq!(heap.iter_level_order().count(), 3);
    /// ```
    pub fn iter_level_order(&self) -> std::slice::Iter<'_, T> {
        self.items.iter()
    }

    fn heapify(&mut self) {
        if self.len() < 2 {
            return;
        }

        let last_parent = (self.len() / 2).saturating_sub(1);
        for index in (0..=last_parent).rev() {
            self.sift_down(index);
        }
    }

    fn sift_up(&mut self, mut index: usize) {
        while index > 0 {
            let parent = parent_index(index);
            if self.items[parent] <= self.items[index] {
                break;
            }

            self.items.swap(parent, index);
            index = parent;
        }
    }

    fn sift_down(&mut self, mut index: usize) {
        loop {
            let left = left_child_index(index);
            let right = right_child_index(index);
            let mut smallest = index;

            if left < self.len() && self.items[left] < self.items[smallest] {
                smallest = left;
            }

            if right < self.len() && self.items[right] < self.items[smallest] {
                smallest = right;
            }

            if smallest == index {
                break;
            }

            self.items.swap(index, smallest);
            index = smallest;
        }
    }
}

impl<T: Ord> Default for MinHeap<T> {
    fn default() -> Self {
        Self::new()
    }
}

fn parent_index(index: usize) -> usize {
    (index - 1) / 2
}

fn left_child_index(index: usize) -> usize {
    (index * 2) + 1
}

fn right_child_index(index: usize) -> usize {
    (index * 2) + 2
}

#[cfg(test)]
mod tests {
    use super::{Heap, MinHeap};

    #[test]
    fn max_heap_root_is_not_smaller_than_children_after_heapify() {
        let heap = Heap::from_vec(vec![4, 1, 7, 3, 9, 2]);
        let values = heap.iter_level_order().copied().collect::<Vec<_>>();

        for index in 0..values.len() {
            let left = (index * 2) + 1;
            let right = (index * 2) + 2;

            if left < values.len() {
                assert!(values[index] >= values[left]);
            }

            if right < values.len() {
                assert!(values[index] >= values[right]);
            }
        }
    }

    #[test]
    fn min_heap_root_is_not_larger_than_children_after_heapify() {
        let heap = MinHeap::from_vec(vec![4, 1, 7, 3, 9, 2]);
        let values = heap.iter_level_order().copied().collect::<Vec<_>>();

        for index in 0..values.len() {
            let left = (index * 2) + 1;
            let right = (index * 2) + 2;

            if left < values.len() {
                assert!(values[index] <= values[left]);
            }

            if right < values.len() {
                assert!(values[index] <= values[right]);
            }
        }
    }
}
