//! Lista enlazada.
//!
//! Este modulo enseñara nodos, enlaces, propiedad de memoria, insercion,
//! eliminacion e iteracion sin esconder los costos de localidad.

/// Lista enlazada simple para estudiar nodos, enlaces y ownership.
///
/// Esta implementacion usa solo Rust seguro. Cada nodo posee su sucesor con
/// `Box<Node<T>>`, y la lista posee el primer nodo mediante `head`.
///
/// ```
/// use rust_data_structures::linked_list::LinkedList;
///
/// let mut list = LinkedList::new();
/// list.push_back("a");
/// list.push_back("b");
///
/// assert_eq!(list.front(), Some(&"a"));
/// assert_eq!(list.back(), Some(&"b"));
/// assert_eq!(list.iter().copied().collect::<Vec<_>>(), vec!["a", "b"]);
/// ```
#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

/// Iterador inmutable sobre una [`LinkedList`].
///
/// ```
/// use rust_data_structures::linked_list::LinkedList;
///
/// let mut list = LinkedList::new();
/// list.push_back(1);
/// list.push_back(2);
///
/// let collected = list.iter().copied().collect::<Vec<_>>();
/// assert_eq!(collected, vec![1, 2]);
/// ```
#[derive(Debug, Clone)]
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<T> LinkedList<T> {
    /// Crea una lista vacia.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::linked_list::LinkedList;
    ///
    /// let list = LinkedList::<i32>::new();
    /// assert!(list.is_empty());
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self { head: None, len: 0 }
    }

    /// Devuelve el numero de elementos.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::linked_list::LinkedList;
    ///
    /// let mut list = LinkedList::new();
    /// list.push_front("nodo");
    ///
    /// assert_eq!(list.len(), 1);
    /// ```
    #[must_use]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Indica si la lista no contiene elementos.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::linked_list::LinkedList;
    ///
    /// let mut list = LinkedList::new();
    /// assert!(list.is_empty());
    ///
    /// list.push_back("dato");
    /// assert!(!list.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Inserta `value` al inicio.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::linked_list::LinkedList;
    ///
    /// let mut list = LinkedList::new();
    /// list.push_front("b");
    /// list.push_front("a");
    ///
    /// assert_eq!(list.iter().copied().collect::<Vec<_>>(), vec!["a", "b"]);
    /// ```
    pub fn push_front(&mut self, value: T) {
        let next = self.head.take();
        self.head = Some(Box::new(Node { value, next }));
        self.len += 1;
    }

    /// Inserta `value` al final.
    ///
    /// Complejidad: O(n), porque esta lista simple no guarda puntero mutable al
    /// ultimo nodo.
    ///
    /// ```
    /// use rust_data_structures::linked_list::LinkedList;
    ///
    /// let mut list = LinkedList::new();
    /// list.push_back("a");
    /// list.push_back("b");
    ///
    /// assert_eq!(list.iter().copied().collect::<Vec<_>>(), vec!["a", "b"]);
    /// ```
    pub fn push_back(&mut self, value: T) {
        let new_node = Box::new(Node { value, next: None });
        let mut cursor = &mut self.head;

        while let Some(node) = cursor {
            cursor = &mut node.next;
        }

        *cursor = Some(new_node);
        self.len += 1;
    }

    /// Remueve y devuelve el primer elemento, si existe.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::linked_list::LinkedList;
    ///
    /// let mut list = LinkedList::new();
    /// list.push_back(10);
    ///
    /// assert_eq!(list.pop_front(), Some(10));
    /// assert_eq!(list.pop_front(), None);
    /// ```
    pub fn pop_front(&mut self) -> Option<T> {
        let head = self.head.take()?;
        self.head = head.next;
        self.len -= 1;
        Some(head.value)
    }

    /// Remueve y devuelve el ultimo elemento, si existe.
    ///
    /// Complejidad: O(n).
    ///
    /// ```
    /// use rust_data_structures::linked_list::LinkedList;
    ///
    /// let mut list = LinkedList::new();
    /// list.push_back(10);
    /// list.push_back(20);
    ///
    /// assert_eq!(list.pop_back(), Some(20));
    /// assert_eq!(list.pop_back(), Some(10));
    /// ```
    pub fn pop_back(&mut self) -> Option<T> {
        match self.len {
            0 => None,
            1 => self.pop_front(),
            _ => {
                let mut current = self.head.as_mut().expect("len > 1 implies head");

                while current
                    .next
                    .as_ref()
                    .and_then(|next| next.next.as_ref())
                    .is_some()
                {
                    current = current.next.as_mut().expect("next exists");
                }

                let tail = current.next.take().expect("tail exists");
                self.len -= 1;
                Some(tail.value)
            }
        }
    }

    /// Devuelve una referencia al primer elemento.
    ///
    /// Complejidad: O(1).
    ///
    /// ```
    /// use rust_data_structures::linked_list::LinkedList;
    ///
    /// let mut list = LinkedList::new();
    /// list.push_front("primero");
    ///
    /// assert_eq!(list.front(), Some(&"primero"));
    /// ```
    #[must_use]
    pub fn front(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    /// Devuelve una referencia al ultimo elemento.
    ///
    /// Complejidad: O(n).
    ///
    /// ```
    /// use rust_data_structures::linked_list::LinkedList;
    ///
    /// let mut list = LinkedList::new();
    /// list.push_back("ultimo");
    ///
    /// assert_eq!(list.back(), Some(&"ultimo"));
    /// ```
    #[must_use]
    pub fn back(&self) -> Option<&T> {
        let mut current = self.head.as_deref()?;

        while let Some(next) = current.next.as_deref() {
            current = next;
        }

        Some(&current.value)
    }

    /// Remueve y devuelve el elemento en `index`, si existe.
    ///
    /// Complejidad: O(n).
    ///
    /// ```
    /// use rust_data_structures::linked_list::LinkedList;
    ///
    /// let mut list = LinkedList::new();
    /// list.push_back("a");
    /// list.push_back("b");
    /// list.push_back("c");
    ///
    /// assert_eq!(list.remove(1), Some("b"));
    /// assert_eq!(list.iter().copied().collect::<Vec<_>>(), vec!["a", "c"]);
    /// ```
    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.len {
            return None;
        }

        if index == 0 {
            return self.pop_front();
        }

        let mut previous = self.head.as_mut().expect("index > 0 implies head");
        for _ in 0..index - 1 {
            previous = previous.next.as_mut().expect("index is inside len");
        }

        let mut removed = previous.next.take().expect("index is inside len");
        previous.next = removed.next.take();
        self.len -= 1;

        Some(removed.value)
    }

    /// Remueve todos los nodos.
    ///
    /// Complejidad: O(n).
    ///
    /// ```
    /// use rust_data_structures::linked_list::LinkedList;
    ///
    /// let mut list = LinkedList::new();
    /// list.push_back(1);
    /// list.clear();
    ///
    /// assert!(list.is_empty());
    /// ```
    pub fn clear(&mut self) {
        while self.pop_front().is_some() {}
    }

    /// Itera sobre referencias a los elementos en orden de enlaces.
    ///
    /// Complejidad: O(1) para crear el iterador y O(n) para consumirlo completo.
    ///
    /// ```
    /// use rust_data_structures::linked_list::LinkedList;
    ///
    /// let mut list = LinkedList::new();
    /// list.push_back(2);
    /// list.push_back(3);
    ///
    /// let sum: i32 = list.iter().copied().sum();
    /// assert_eq!(sum, 5);
    /// ```
    #[must_use]
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.next?;
        self.next = node.next.as_deref();
        Some(&node.value)
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn len_matches_number_of_links_after_mixed_operations() {
        let mut list = LinkedList::new();

        list.push_front(2);
        list.push_front(1);
        list.push_back(3);
        list.remove(1);

        assert_eq!(list.len(), 2);
        assert_eq!(list.iter().copied().collect::<Vec<_>>(), vec![1, 3]);
    }

    #[test]
    fn clear_is_idempotent() {
        let mut list = LinkedList::new();

        list.push_back("a");
        list.clear();
        list.clear();

        assert!(list.is_empty());
        assert_eq!(list.pop_front(), None);
        assert_eq!(list.pop_back(), None);
    }
}
