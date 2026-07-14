use rust_data_structures::deque::Deque;

fn main() {
    let mut history = Deque::new();
    history.push_back("inicio");
    history.push_back("curso");
    history.push_back("capitulo");

    assert_eq!(go_back(&mut history), Some("capitulo"));
    history.push_front("pin-roadmap");

    assert_eq!(
        history.iter().copied().collect::<Vec<_>>(),
        vec!["pin-roadmap", "inicio", "curso"]
    );
    println!("{:?}", history.iter().copied().collect::<Vec<_>>());
}

fn go_back(history: &mut Deque<&'static str>) -> Option<&'static str> {
    history.pop_back()
}
