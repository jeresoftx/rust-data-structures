use rust_data_structures::graph::Graph;

fn main() {
    let mut dependencies = Graph::new_directed();

    dependencies.add_node("lexer");
    dependencies.add_node("parser");
    dependencies.add_node("type-checker");

    dependencies.add_edge("parser", "lexer", 1).unwrap();
    dependencies.add_edge("type-checker", "parser", 1).unwrap();

    println!("nodos: {}", dependencies.node_count());
    println!("aristas: {}", dependencies.edge_count());
    println!(
        "parser depende de lexer: {}",
        dependencies.has_edge("parser", "lexer")
    );
    println!(
        "lexer depende de parser: {}",
        dependencies.has_edge("lexer", "parser")
    );
}
