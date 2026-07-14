use rust_data_structures::heap::MinHeap;

fn main() {
    let mut frontier = MinHeap::new();

    frontier.push(State::new(0, "inicio"));
    frontier.push(State::new(7, "cache"));
    frontier.push(State::new(3, "api"));

    while let Some(state) = frontier.pop() {
        println!("visitar {} con costo {}", state.node, state.cost);
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct State {
    cost: u32,
    node: &'static str,
}

impl State {
    fn new(cost: u32, node: &'static str) -> Self {
        Self { cost, node }
    }
}
