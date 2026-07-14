use rust_data_structures::graph::AdjacencyMatrix;

fn main() {
    let mut routes = AdjacencyMatrix::new_directed();

    for city in ["mxn", "gdl", "qro"] {
        routes.add_node(city);
    }

    routes.add_edge("mxn", "gdl", 9).unwrap();
    routes.add_edge("gdl", "qro", 5).unwrap();

    assert_eq!(routes.edge_weight("mxn", "gdl"), Some(9));
    assert!(!routes.has_edge("qro", "mxn"));
    assert_eq!(routes.node_count(), 3);

    println!("mxn -> gdl: {:?}", routes.edge_weight("mxn", "gdl"));
}
