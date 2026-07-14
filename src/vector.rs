//! Vector: arreglo dinamico contiguo.
//!
//! Este modulo enseñara capacidad, longitud, crecimiento amortizado, acceso por
//! indice y las compensaciones de memoria contigua.

/// Arreglo dinamico contiguo para estudiar longitud, capacidad y crecimiento.
///
/// Esta primera implementacion prioriza invariantes visibles y Rust seguro. Usa
/// `Option<T>` como celda inicializada para evitar gestion manual de memoria; el
/// capitulo explica que una implementacion industrial usa memoria sin inicializar
/// y requiere `unsafe` cuidadosamente justificado.
#[derive(Debug)]
pub struct Vector<T> {
    items: Box<[Option<T>]>,
    len: usize,
}

/// Error al insertar en una posicion que no existe.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct InsertError {
    /// Indice solicitado por quien llama.
    pub index: usize,
    /// Longitud del vector en el momento de la operacion.
    pub len: usize,
}

impl<T> Vector<T> {
    /// Crea un vector vacio sin capacidad reservada.
    ///
    /// Complejidad: O(1) tiempo y O(1) espacio.
    #[must_use]
    pub fn new() -> Self {
        Self {
            items: allocate_slots(0),
            len: 0,
        }
    }

    /// Crea un vector vacio con capacidad para `capacity` elementos.
    ///
    /// Complejidad: O(n) tiempo y O(n) espacio, donde `n` es `capacity`, porque
    /// esta version segura inicializa cada celda con `None`.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            items: allocate_slots(capacity),
            len: 0,
        }
    }

    /// Devuelve el numero de elementos almacenados.
    ///
    /// Complejidad: O(1).
    #[must_use]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Devuelve la cantidad de elementos que caben sin crecer.
    ///
    /// Complejidad: O(1).
    #[must_use]
    pub fn capacity(&self) -> usize {
        self.items.len()
    }

    /// Indica si el vector no contiene elementos.
    ///
    /// Complejidad: O(1).
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Agrega `value` al final del vector.
    ///
    /// Complejidad: O(1) amortizado. Cuando la capacidad se agota, crecer cuesta
    /// O(n) porque se mueven los elementos al nuevo bloque.
    pub fn push(&mut self, value: T) {
        if self.len == self.capacity() {
            self.grow();
        }

        self.items[self.len] = Some(value);
        self.len += 1;
    }

    /// Remueve y devuelve el ultimo elemento, si existe.
    ///
    /// Complejidad: O(1).
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        self.len -= 1;
        self.items[self.len].take()
    }

    /// Devuelve una referencia al elemento en `index`, si existe.
    ///
    /// Complejidad: O(1).
    #[must_use]
    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.len {
            return None;
        }

        self.items[index].as_ref()
    }

    /// Devuelve una referencia mutable al elemento en `index`, si existe.
    ///
    /// Complejidad: O(1).
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index >= self.len {
            return None;
        }

        self.items[index].as_mut()
    }

    /// Inserta `value` en `index`, desplazando a la derecha los elementos
    /// posteriores.
    ///
    /// `index == len` es valido y equivale a `push`. Un indice mayor que `len`
    /// devuelve `InsertError`.
    ///
    /// Complejidad: O(n) en el peor caso por el desplazamiento.
    pub fn insert(&mut self, index: usize, value: T) -> Result<(), InsertError> {
        if index > self.len {
            return Err(InsertError {
                index,
                len: self.len,
            });
        }

        if self.len == self.capacity() {
            self.grow();
        }

        for current in (index..self.len).rev() {
            self.items[current + 1] = self.items[current].take();
        }

        self.items[index] = Some(value);
        self.len += 1;

        Ok(())
    }

    /// Remueve y devuelve el elemento en `index`, si existe.
    ///
    /// Complejidad: O(n) en el peor caso por el desplazamiento.
    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.len {
            return None;
        }

        let removed = self.items[index].take();

        for current in index + 1..self.len {
            self.items[current - 1] = self.items[current].take();
        }

        self.len -= 1;
        self.items[self.len] = None;

        removed
    }

    /// Remueve todos los elementos sin liberar la capacidad reservada.
    ///
    /// Complejidad: O(n).
    pub fn clear(&mut self) {
        for index in 0..self.len {
            self.items[index] = None;
        }

        self.len = 0;
    }

    /// Itera sobre referencias a los elementos almacenados.
    ///
    /// Complejidad: O(1) para crear el iterador y O(n) para consumirlo completo.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.items[..self.len]
            .iter()
            .map(|slot| slot.as_ref().expect("occupied slots before len"))
    }

    fn grow(&mut self) {
        let next_capacity = if self.capacity() == 0 {
            1
        } else {
            self.capacity() * 2
        };

        let mut next_items = allocate_slots(next_capacity);
        for index in 0..self.len {
            next_items[index] = self.items[index].take();
        }

        self.items = next_items;
    }
}

impl<T> Default for Vector<T> {
    fn default() -> Self {
        Self::new()
    }
}

fn allocate_slots<T>(capacity: usize) -> Box<[Option<T>]> {
    std::iter::repeat_with(|| None)
        .take(capacity)
        .collect::<Vec<_>>()
        .into_boxed_slice()
}
