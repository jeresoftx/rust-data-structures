use rust_data_structures::graph::Graph;

fn main() {
    let mut course_map = Graph::new_directed();

    for course in [
        "rust-basics",
        "ownership",
        "data-structures",
        "algorithms",
        "distributed-systems",
    ] {
        course_map.add_node(course);
    }

    course_map.add_edge("ownership", "rust-basics", 1).unwrap();
    course_map
        .add_edge("data-structures", "ownership", 1)
        .unwrap();
    course_map
        .add_edge("algorithms", "data-structures", 1)
        .unwrap();
    course_map
        .add_edge("distributed-systems", "algorithms", 1)
        .unwrap();

    for edge in course_map.edges() {
        println!("{} requiere {}", edge.from, edge.to);
    }
}
