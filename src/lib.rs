//! Estructuras de datos en Rust para Jeresoft Academy.
//!
//! Este crate acompaña el curso `rust-data-structures`, parte del Semestre 1
//! del camino troncal de Jeresoft Academy. Cada modulo representa una
//! estructura de datos canonica: su representacion, invariantes, operaciones,
//! pruebas, benchmarks y capitulo educativo.
//!
//! El objetivo del crate no es reemplazar las colecciones de la biblioteca
//! estandar. Es explicar como se diseñan y verifican las estructuras que luego
//! aparecen en algoritmos, bases de datos, sistemas distribuidos y arquitectura
//! de software.

#![forbid(unsafe_code)]

/// Arreglo dinamico contiguo con crecimiento de capacidad.
pub mod vector;

/// Lista enlazada como introduccion a nodos, ownership y punteros.
pub mod linked_list;

/// Pila LIFO construida como abstraccion sobre almacenamiento interno.
pub mod stack;

/// Cola FIFO para procesar elementos en orden de llegada.
pub mod queue;

/// Cola de doble extremo para insertar y remover por ambos lados.
pub mod deque;

/// Monticulo binario para prioridades y seleccion eficiente de extremos.
pub mod heap;

/// Arbol de prefijos para busqueda por cadenas y autocompletado.
pub mod trie;

/// Representaciones de grafos y operaciones basicas sobre vertices y aristas.
pub mod graph;

/// Arbol B para mapas ordenados con buena localidad de memoria.
pub mod btree;

/// Tabla hash para busqueda promedio constante por clave.
pub mod hashmap;

/// Filtro de Bloom para pertenencia probabilistica.
pub mod bloom_filter;

/// Skip list para conjuntos o mapas ordenados con balance probabilistico.
pub mod skip_list;
