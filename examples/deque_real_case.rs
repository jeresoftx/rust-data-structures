use rust_data_structures::deque::Deque;

fn main() {
    let mut tabs = Deque::new();

    tabs.push_back(Tab::new("Inicio"));
    tabs.push_back(Tab::new("Curso"));
    tabs.push_front(Tab::new("Pin: Roadmap"));

    while let Some(tab) = tabs.pop_front() {
        println!("visitando: {}", tab.title);
    }
}

struct Tab {
    title: &'static str,
}

impl Tab {
    fn new(title: &'static str) -> Self {
        Self { title }
    }
}
