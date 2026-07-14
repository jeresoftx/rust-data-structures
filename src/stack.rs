//! Pila.
//!
//! Este modulo enseñara la abstraccion LIFO, sus operaciones basicas y las
//! diferencias entre representar una pila con vector o con nodos enlazados.

use crate::vector::Vector;

/// Pila LIFO respaldada por un [`Vector`].
///
/// El tope de la pila vive al final del vector. Esa decision hace que `push`,
/// `pop` y `peek` sean operaciones constantes o amortizadas constantes.
///
/// ```
/// use rust_data_structures::stack::Stack;
///
/// let mut stack = Stack::new();
/// stack.push("base");
/// stack.push("top");
///
/// assert_eq!(stack.peek(), Some(&"top"));
/// assert_eq!(stack.pop(), Some("top"));
/// assert_eq!(stack.pop(), Some("base"));
/// ```
#[derive(Debug)]
pub struct Stack<T> {
    items: Vector<T>,
}

impl<T> Stack<T> {
    /// Crea una pila vacia.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::stack::Stack;
    ///
    /// let stack = Stack::<i32>::new();
    /// assert!(stack.is_empty());
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            items: Vector::new(),
        }
    }

    /// Crea una pila vacia con capacidad reservada.
    ///
    /// Complejidad: O(n) por la inicializacion segura del `Vector` interno.
    ///
    /// ```
    /// use rust_data_structures::stack::Stack;
    ///
    /// let stack = Stack::<String>::with_capacity(8);
    /// assert_eq!(stack.capacity(), 8);
    /// ```
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            items: Vector::with_capacity(capacity),
        }
    }

    /// Devuelve el numero de elementos.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::stack::Stack;
    ///
    /// let mut stack = Stack::new();
    /// stack.push("dato");
    /// assert_eq!(stack.len(), 1);
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
    /// use rust_data_structures::stack::Stack;
    ///
    /// let stack = Stack::<u8>::with_capacity(2);
    /// assert_eq!(stack.capacity(), 2);
    /// ```
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.items.capacity()
    }

    /// Indica si la pila esta vacia.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::stack::Stack;
    ///
    /// let mut stack = Stack::new();
    /// assert!(stack.is_empty());
    /// stack.push(1);
    /// assert!(!stack.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Inserta `value` en el tope.
    ///
    /// Complejidad: O(1) amortizado.
    ///
    /// ```
    /// use rust_data_structures::stack::Stack;
    ///
    /// let mut stack = Stack::new();
    /// stack.push("undo");
    ///
    /// assert_eq!(stack.peek(), Some(&"undo"));
    /// ```
    pub fn push(&mut self, value: T) {
        self.items.push(value);
    }

    /// Remueve y devuelve el tope, si existe.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::stack::Stack;
    ///
    /// let mut stack = Stack::new();
    /// stack.push(10);
    ///
    /// assert_eq!(stack.pop(), Some(10));
    /// assert_eq!(stack.pop(), None);
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    /// Devuelve una referencia al tope, si existe.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::stack::Stack;
    ///
    /// let mut stack = Stack::new();
    /// stack.push("visible");
    ///
    /// assert_eq!(stack.peek(), Some(&"visible"));
    /// ```
    #[must_use]
    pub fn peek(&self) -> Option<&T> {
        self.items
            .len()
            .checked_sub(1)
            .and_then(|index| self.items.get(index))
    }

    /// Devuelve una referencia mutable al tope, si existe.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::stack::Stack;
    ///
    /// let mut stack = Stack::new();
    /// stack.push(String::from("draft"));
    ///
    /// stack.peek_mut().unwrap().push_str("-done");
    /// assert_eq!(stack.peek().map(String::as_str), Some("draft-done"));
    /// ```
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.items
            .len()
            .checked_sub(1)
            .and_then(|index| self.items.get_mut(index))
    }

    /// Remueve todos los elementos sin liberar la capacidad reservada.
    ///
    /// Complejidad: O(n).
    ///
    /// ```
    /// use rust_data_structures::stack::Stack;
    ///
    /// let mut stack = Stack::with_capacity(4);
    /// stack.push(1);
    /// stack.clear();
    ///
    /// assert!(stack.is_empty());
    /// assert_eq!(stack.capacity(), 4);
    /// ```
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Itera desde la base hasta el tope.
    ///
    /// Complejidad: O(1) para crear el iterador y O(n) para consumirlo completo.
    ///
    /// ```
    /// use rust_data_structures::stack::Stack;
    ///
    /// let mut stack = Stack::new();
    /// stack.push(1);
    /// stack.push(2);
    ///
    /// assert_eq!(stack.iter().copied().collect::<Vec<_>>(), vec![1, 2]);
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.items.iter()
    }
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::Stack;

    #[test]
    fn top_is_always_last_pushed_value() {
        let mut stack = Stack::new();

        stack.push("a");
        assert_eq!(stack.peek(), Some(&"a"));

        stack.push("b");
        assert_eq!(stack.peek(), Some(&"b"));
    }

    #[test]
    fn clear_is_idempotent() {
        let mut stack = Stack::new();

        stack.push(1);
        stack.clear();
        stack.clear();

        assert!(stack.is_empty());
        assert_eq!(stack.pop(), None);
    }
}
