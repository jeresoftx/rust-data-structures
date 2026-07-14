use rust_data_structures::graph::Graph;

fn main() {
    let mut social = Graph::new_undirected();

    for person in ["ana", "beatriz", "carlos", "diego"] {
        social.add_node(person);
    }

    social.add_edge("ana", "beatriz", 5).unwrap();
    social.add_edge("ana", "carlos", 2).unwrap();
    social.add_edge("beatriz", "diego", 1).unwrap();

    println!("amistades logicas: {}", social.edge_count());
    println!("vecinos de ana: {:?}", social.neighbors("ana").unwrap());
    println!("beatriz ve a ana: {}", social.has_edge("beatriz", "ana"));
}
