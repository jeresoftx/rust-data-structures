use rust_data_structures::graph::Graph;

fn main() {
    let mut graph = Graph::new_undirected();

    for person in ["ana", "beatriz", "carlos", "diego"] {
        graph.add_node(person);
    }

    graph.add_edge("ana", "beatriz", 10).unwrap();
    graph.add_edge("ana", "carlos", 4).unwrap();
    graph.add_edge("diego", "beatriz", 2).unwrap();

    let ana_neighbors = graph.neighbors("ana").unwrap();

    assert_eq!(ana_neighbors, vec![("beatriz", 10), ("carlos", 4)]);
    assert!(graph.has_edge("beatriz", "ana"));
    assert_eq!(graph.edge_count(), 3);

    println!("vecinos de ana: {ana_neighbors:?}");
}
