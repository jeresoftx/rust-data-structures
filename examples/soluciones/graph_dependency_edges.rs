use rust_data_structures::graph::Graph;

fn main() {
    let mut graph = Graph::new_directed();

    for crate_name in ["api", "domain", "database", "observability"] {
        graph.add_node(crate_name);
    }

    graph.add_edge("api", "domain", 1).unwrap();
    graph.add_edge("api", "observability", 1).unwrap();
    graph.add_edge("database", "domain", 1).unwrap();

    let edges = graph.edges();

    assert_eq!(edges.len(), 3);
    assert!(graph.has_edge("api", "domain"));
    assert!(!graph.has_edge("domain", "api"));

    for edge in edges {
        println!("{} -> {}", edge.from, edge.to);
    }
}
