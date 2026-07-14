use rust_data_structures::graph::{AdjacencyMatrix, Graph, GraphError};

#[test]
fn directed_graph_keeps_edge_direction_and_weight() {
    let mut graph = Graph::new_directed();

    assert!(graph.is_directed());
    assert!(graph.add_node("parser"));
    assert!(graph.add_node("lexer"));
    assert!(!graph.add_node("parser"));

    assert_eq!(graph.add_edge("parser", "lexer", 3), Ok(true));
    assert_eq!(graph.add_edge("parser", "lexer", 5), Ok(false));

    assert!(graph.has_edge("parser", "lexer"));
    assert!(!graph.has_edge("lexer", "parser"));
    assert_eq!(graph.edge_weight("parser", "lexer"), Some(5));
    assert_eq!(graph.node_count(), 2);
    assert_eq!(graph.edge_count(), 1);
}

#[test]
fn undirected_graph_mirrors_edges_but_counts_them_once() {
    let mut graph = Graph::new_undirected();

    graph.add_node("a");
    graph.add_node("b");
    graph.add_node("c");

    assert_eq!(graph.add_edge("a", "b", 7), Ok(true));
    assert_eq!(graph.add_edge("b", "a", 9), Ok(false));

    assert!(graph.has_edge("a", "b"));
    assert!(graph.has_edge("b", "a"));
    assert_eq!(graph.edge_weight("a", "b"), Some(9));
    assert_eq!(graph.edge_weight("b", "a"), Some(9));
    assert_eq!(graph.edge_count(), 1);
    assert_eq!(graph.neighbors("a"), Some(vec![("b", 9)]));
}

#[test]
fn self_loops_are_supported_and_count_once() {
    let mut graph = Graph::new_undirected();

    graph.add_node("home");
    assert_eq!(graph.add_edge("home", "home", 1), Ok(true));

    assert!(graph.has_edge("home", "home"));
    assert_eq!(graph.edge_count(), 1);
    assert_eq!(graph.neighbors("home"), Some(vec![("home", 1)]));
}

#[test]
fn missing_nodes_are_reported_when_adding_edges() {
    let mut graph = Graph::new_directed();

    graph.add_node("known");

    assert_eq!(
        graph.add_edge("known", "missing", 1),
        Err(GraphError::MissingNode)
    );
    assert_eq!(
        graph.add_edge("missing", "known", 1),
        Err(GraphError::MissingNode)
    );
    assert_eq!(graph.edge_count(), 0);
}

#[test]
fn removing_edges_and_nodes_updates_incident_edges() {
    let mut graph = Graph::new_directed();

    graph.add_node("a");
    graph.add_node("b");
    graph.add_node("c");
    graph.add_edge("a", "b", 1).unwrap();
    graph.add_edge("b", "c", 1).unwrap();
    graph.add_edge("c", "a", 1).unwrap();

    assert!(graph.remove_edge("a", "b"));
    assert!(!graph.has_edge("a", "b"));
    assert_eq!(graph.edge_count(), 2);

    assert!(graph.remove_node("c"));
    assert!(!graph.contains_node("c"));
    assert!(!graph.has_edge("b", "c"));
    assert_eq!(graph.edge_count(), 0);
}

#[test]
fn removing_node_from_undirected_graph_removes_incident_edges_once() {
    let mut graph = Graph::new_undirected();

    graph.add_node("a");
    graph.add_node("b");
    graph.add_node("c");
    graph.add_edge("a", "b", 1).unwrap();
    graph.add_edge("b", "c", 2).unwrap();

    assert!(graph.remove_node("b"));

    assert_eq!(graph.edge_count(), 0);
    assert!(!graph.has_edge("a", "b"));
    assert!(!graph.has_edge("c", "b"));
    assert!(graph.contains_node("a"));
    assert!(graph.contains_node("c"));
}

#[test]
fn edge_list_is_deterministic() {
    let mut graph = Graph::new_directed();

    for node in ["b", "a", "c"] {
        graph.add_node(node);
    }
    graph.add_edge("b", "c", 2).unwrap();
    graph.add_edge("a", "b", 1).unwrap();

    let edges = graph.edges();

    assert_eq!(edges.len(), 2);
    assert_eq!(edges[0].from, "a");
    assert_eq!(edges[0].to, "b");
    assert_eq!(edges[1].from, "b");
    assert_eq!(edges[1].to, "c");
}

#[test]
fn adjacency_matrix_represents_dense_directed_graphs() {
    let mut matrix = AdjacencyMatrix::new_directed();

    matrix.add_node("a");
    matrix.add_node("b");
    matrix.add_node("c");

    assert_eq!(matrix.add_edge("a", "b", 4), Ok(true));
    assert_eq!(matrix.add_edge("b", "a", 8), Ok(true));

    assert!(matrix.has_edge("a", "b"));
    assert!(matrix.has_edge("b", "a"));
    assert_eq!(matrix.edge_weight("a", "b"), Some(4));
    assert_eq!(matrix.edge_count(), 2);
    assert_eq!(matrix.node_count(), 3);
}
