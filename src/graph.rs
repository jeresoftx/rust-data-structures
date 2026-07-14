//! Grafo.
//!
//! Este modulo enseñara representaciones de grafos: lista de adyacencia, matriz
//! de adyacencia y lista de aristas. Los algoritmos profundos viven en
//! `rust-algorithms`.

use std::collections::BTreeMap;

/// Peso entero usado por las aristas de este capitulo.
pub type Weight = i32;

/// Error al operar sobre grafos.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GraphError {
    /// La operacion referencia un nodo que no existe.
    MissingNode,
}

/// Arista ponderada en forma de lista de aristas.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Edge<T> {
    /// Nodo origen.
    pub from: T,
    /// Nodo destino.
    pub to: T,
    /// Peso asociado.
    pub weight: Weight,
}

/// Grafo ponderado respaldado por lista de adyacencia.
///
/// ```
/// use rust_data_structures::graph::Graph;
///
/// let mut graph = Graph::new_directed();
/// graph.add_node("parser");
/// graph.add_node("lexer");
/// graph.add_edge("parser", "lexer", 1).unwrap();
///
/// assert!(graph.has_edge("parser", "lexer"));
/// assert!(!graph.has_edge("lexer", "parser"));
/// ```
#[derive(Debug, Clone)]
pub struct Graph<T> {
    directed: bool,
    adjacency: BTreeMap<T, BTreeMap<T, Weight>>,
    edge_count: usize,
}

/// Grafo ponderado respaldado por matriz de adyacencia.
///
/// ```
/// use rust_data_structures::graph::AdjacencyMatrix;
///
/// let mut graph = AdjacencyMatrix::new_undirected();
/// graph.add_node("a");
/// graph.add_node("b");
/// graph.add_edge("a", "b", 1).unwrap();
///
/// assert!(graph.has_edge("b", "a"));
/// ```
#[derive(Debug, Clone)]
pub struct AdjacencyMatrix<T> {
    directed: bool,
    nodes: Vec<T>,
    weights: Vec<Vec<Option<Weight>>>,
    edge_count: usize,
}

impl<T: Ord + Clone> Graph<T> {
    /// Crea un grafo dirigido.
    ///
    /// Complejidad: O(1).
    #[must_use]
    pub fn new_directed() -> Self {
        Self {
            directed: true,
            adjacency: BTreeMap::new(),
            edge_count: 0,
        }
    }

    /// Crea un grafo no dirigido.
    ///
    /// Complejidad: O(1).
    #[must_use]
    pub fn new_undirected() -> Self {
        Self {
            directed: false,
            adjacency: BTreeMap::new(),
            edge_count: 0,
        }
    }

    /// Indica si el grafo es dirigido.
    #[must_use]
    pub fn is_directed(&self) -> bool {
        self.directed
    }

    /// Agrega un nodo y devuelve `true` si era nuevo.
    ///
    /// Complejidad: O(log n).
    pub fn add_node(&mut self, node: T) -> bool {
        if self.adjacency.contains_key(&node) {
            return false;
        }

        self.adjacency.insert(node, BTreeMap::new());
        true
    }

    /// Indica si un nodo existe.
    ///
    /// Complejidad: O(log n).
    #[must_use]
    pub fn contains_node(&self, node: T) -> bool {
        self.adjacency.contains_key(&node)
    }

    /// Devuelve el numero de nodos.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.adjacency.len()
    }

    /// Devuelve el numero logico de aristas.
    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.edge_count
    }

    /// Agrega o actualiza una arista.
    ///
    /// Devuelve `true` si la arista era nueva y `false` si solo actualizo peso.
    ///
    /// Complejidad: O(log n).
    pub fn add_edge(&mut self, from: T, to: T, weight: Weight) -> Result<bool, GraphError> {
        if !self.adjacency.contains_key(&from) || !self.adjacency.contains_key(&to) {
            return Err(GraphError::MissingNode);
        }

        let inserted = self
            .adjacency
            .get_mut(&from)
            .expect("source node exists")
            .insert(to.clone(), weight)
            .is_none();

        if !self.directed && from != to {
            self.adjacency
                .get_mut(&to)
                .expect("target node exists")
                .insert(from, weight);
        }

        if inserted {
            self.edge_count += 1;
        }

        Ok(inserted)
    }

    /// Remueve una arista, si existe.
    ///
    /// Complejidad: O(log n).
    pub fn remove_edge(&mut self, from: T, to: T) -> bool {
        let removed = self
            .adjacency
            .get_mut(&from)
            .and_then(|neighbors| neighbors.remove(&to))
            .is_some();

        if removed {
            if !self.directed && from != to {
                if let Some(neighbors) = self.adjacency.get_mut(&to) {
                    neighbors.remove(&from);
                }
            }
            self.edge_count -= 1;
        }

        removed
    }

    /// Remueve un nodo y todas sus aristas incidentes.
    ///
    /// Complejidad: O(n log n + e).
    pub fn remove_node(&mut self, node: T) -> bool {
        let Some(outgoing) = self.adjacency.remove(&node) else {
            return false;
        };

        if self.directed {
            self.edge_count -= outgoing.len();

            for neighbors in self.adjacency.values_mut() {
                if neighbors.remove(&node).is_some() {
                    self.edge_count -= 1;
                }
            }
        } else {
            self.edge_count -= outgoing.len();

            for neighbors in self.adjacency.values_mut() {
                neighbors.remove(&node);
            }
        }

        true
    }

    /// Indica si una arista existe.
    ///
    /// Complejidad: O(log n).
    #[must_use]
    pub fn has_edge(&self, from: T, to: T) -> bool {
        self.adjacency
            .get(&from)
            .is_some_and(|neighbors| neighbors.contains_key(&to))
    }

    /// Devuelve el peso de una arista.
    ///
    /// Complejidad: O(log n).
    #[must_use]
    pub fn edge_weight(&self, from: T, to: T) -> Option<Weight> {
        self.adjacency
            .get(&from)
            .and_then(|neighbors| neighbors.get(&to))
            .copied()
    }

    /// Devuelve vecinos en orden determinista.
    ///
    /// Complejidad: O(d), donde `d` es el grado de salida.
    #[must_use]
    pub fn neighbors(&self, node: T) -> Option<Vec<(T, Weight)>> {
        self.adjacency.get(&node).map(|neighbors| {
            neighbors
                .iter()
                .map(|(neighbor, weight)| (neighbor.clone(), *weight))
                .collect()
        })
    }

    /// Devuelve una lista de aristas en orden determinista.
    ///
    /// Complejidad: O(n + e).
    #[must_use]
    pub fn edges(&self) -> Vec<Edge<T>> {
        let mut edges = Vec::new();

        for (from, neighbors) in &self.adjacency {
            for (to, weight) in neighbors {
                if !self.directed && from > to {
                    continue;
                }

                edges.push(Edge {
                    from: from.clone(),
                    to: to.clone(),
                    weight: *weight,
                });
            }
        }

        edges
    }
}

impl<T: Ord + Clone> AdjacencyMatrix<T> {
    /// Crea una matriz dirigida.
    #[must_use]
    pub fn new_directed() -> Self {
        Self {
            directed: true,
            nodes: Vec::new(),
            weights: Vec::new(),
            edge_count: 0,
        }
    }

    /// Crea una matriz no dirigida.
    #[must_use]
    pub fn new_undirected() -> Self {
        Self {
            directed: false,
            nodes: Vec::new(),
            weights: Vec::new(),
            edge_count: 0,
        }
    }

    /// Indica si la matriz representa un grafo dirigido.
    #[must_use]
    pub fn is_directed(&self) -> bool {
        self.directed
    }

    /// Agrega un nodo y expande la matriz.
    ///
    /// Complejidad: O(n^2) por realocacion de filas.
    pub fn add_node(&mut self, node: T) -> bool {
        if self.index_of(&node).is_some() {
            return false;
        }

        self.nodes.push(node);
        for row in &mut self.weights {
            row.push(None);
        }
        self.weights.push(vec![None; self.nodes.len()]);
        true
    }

    /// Devuelve el numero de nodos.
    #[must_use]
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Devuelve el numero logico de aristas.
    #[must_use]
    pub fn edge_count(&self) -> usize {
        self.edge_count
    }

    /// Agrega o actualiza una arista.
    pub fn add_edge(&mut self, from: T, to: T, weight: Weight) -> Result<bool, GraphError> {
        let Some(from_index) = self.index_of(&from) else {
            return Err(GraphError::MissingNode);
        };
        let Some(to_index) = self.index_of(&to) else {
            return Err(GraphError::MissingNode);
        };

        let inserted = self.weights[from_index][to_index].is_none();
        self.weights[from_index][to_index] = Some(weight);

        if !self.directed && from_index != to_index {
            self.weights[to_index][from_index] = Some(weight);
        }

        if inserted {
            self.edge_count += 1;
        }

        Ok(inserted)
    }

    /// Indica si una arista existe.
    #[must_use]
    pub fn has_edge(&self, from: T, to: T) -> bool {
        self.edge_weight(from, to).is_some()
    }

    /// Devuelve el peso de una arista.
    #[must_use]
    pub fn edge_weight(&self, from: T, to: T) -> Option<Weight> {
        let from_index = self.index_of(&from)?;
        let to_index = self.index_of(&to)?;
        self.weights[from_index][to_index]
    }

    fn index_of(&self, node: &T) -> Option<usize> {
        self.nodes.iter().position(|candidate| candidate == node)
    }
}

#[cfg(test)]
mod tests {
    use super::{AdjacencyMatrix, Graph};

    #[test]
    fn undirected_edge_list_does_not_duplicate_mirrored_edges() {
        let mut graph = Graph::new_undirected();

        graph.add_node(1);
        graph.add_node(2);
        graph.add_edge(1, 2, 10).unwrap();

        assert_eq!(graph.edges().len(), 1);
    }

    #[test]
    fn undirected_matrix_mirrors_edge_weight() {
        let mut matrix = AdjacencyMatrix::new_undirected();

        matrix.add_node("a");
        matrix.add_node("b");
        matrix.add_edge("a", "b", 2).unwrap();

        assert_eq!(matrix.edge_weight("b", "a"), Some(2));
        assert_eq!(matrix.edge_count(), 1);
    }
}
