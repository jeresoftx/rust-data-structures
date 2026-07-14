use rust_data_structures::graph::{AdjacencyMatrix, Graph};

fn main() {
    let nodes = ["a", "b", "c", "d"];
    let routes = [
        ("a", "b", 10),
        ("a", "c", 15),
        ("b", "c", 3),
        ("c", "d", 7),
        ("d", "a", 20),
    ];

    let mut list = Graph::new_directed();
    let mut matrix = AdjacencyMatrix::new_directed();

    for node in nodes {
        list.add_node(node);
        matrix.add_node(node);
    }

    for (from, to, weight) in routes {
        list.add_edge(from, to, weight).unwrap();
        matrix.add_edge(from, to, weight).unwrap();
    }

    println!("lista: vecinos de a = {:?}", list.neighbors("a").unwrap());
    println!("matriz: peso a->c = {:?}", matrix.edge_weight("a", "c"));
    println!("matriz: existe b->d = {}", matrix.has_edge("b", "d"));
}
