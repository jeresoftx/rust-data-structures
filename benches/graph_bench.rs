use rust_data_structures::graph::{AdjacencyMatrix, Graph};
use std::hint::black_box;
use std::time::{Duration, Instant};

const SIZE: i32 = 700;

fn main() {
    println!("graph benchmark (manual, std::time::Instant)");
    println!("size: {SIZE}");
    println!(
        "adjacency list sparse build: {:?}",
        bench_list_sparse_build()
    );
    println!(
        "adjacency matrix sparse build: {:?}",
        bench_matrix_sparse_build()
    );
    println!("adjacency list edge query: {:?}", bench_list_edge_query());
    println!(
        "adjacency matrix edge query: {:?}",
        bench_matrix_edge_query()
    );
    println!(
        "adjacency list neighbor scan: {:?}",
        bench_list_neighbor_scan()
    );
}

fn bench_list_sparse_build() -> Duration {
    let start = Instant::now();
    black_box(build_list());
    start.elapsed()
}

fn bench_matrix_sparse_build() -> Duration {
    let start = Instant::now();
    black_box(build_matrix());
    start.elapsed()
}

fn bench_list_edge_query() -> Duration {
    let graph = build_list();

    let start = Instant::now();
    for node in 0..SIZE {
        black_box(graph.has_edge(node, (node + 1) % SIZE));
    }
    start.elapsed()
}

fn bench_matrix_edge_query() -> Duration {
    let graph = build_matrix();

    let start = Instant::now();
    for node in 0..SIZE {
        black_box(graph.has_edge(node, (node + 1) % SIZE));
    }
    start.elapsed()
}

fn bench_list_neighbor_scan() -> Duration {
    let graph = build_list();

    let start = Instant::now();
    for node in 0..SIZE {
        black_box(graph.neighbors(node));
    }
    start.elapsed()
}

fn build_list() -> Graph<i32> {
    let mut graph = Graph::new_directed();

    for node in 0..SIZE {
        graph.add_node(node);
    }

    for node in 0..SIZE {
        graph.add_edge(node, (node + 1) % SIZE, 1).unwrap();
        graph.add_edge(node, (node + 7) % SIZE, 1).unwrap();
    }

    graph
}

fn build_matrix() -> AdjacencyMatrix<i32> {
    let mut graph = AdjacencyMatrix::new_directed();

    for node in 0..SIZE {
        graph.add_node(node);
    }

    for node in 0..SIZE {
        graph.add_edge(node, (node + 1) % SIZE, 1).unwrap();
        graph.add_edge(node, (node + 7) % SIZE, 1).unwrap();
    }

    graph
}
