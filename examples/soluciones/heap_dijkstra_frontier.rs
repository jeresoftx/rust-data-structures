use rust_data_structures::heap::MinHeap;

fn main() {
    let mut frontier = MinHeap::new();
    frontier.push(State::new(10, "lento"));
    frontier.push(State::new(2, "rapido"));
    frontier.push(State::new(5, "medio"));

    assert_eq!(frontier.pop().map(|state| state.node), Some("rapido"));
    assert_eq!(frontier.pop().map(|state| state.node), Some("medio"));
    assert_eq!(frontier.pop().map(|state| state.node), Some("lento"));
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
