use rust_data_structures::heap::MinHeap;

fn main() {
    let mut scheduler = MinHeap::new();

    scheduler.push(Task::new(15, "generar reporte"));
    scheduler.push(Task::new(5, "responder healthcheck"));
    scheduler.push(Task::new(30, "compactar logs"));

    while let Some(task) = scheduler.pop() {
        println!("{} ms: {}", task.deadline_ms, task.name);
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Task {
    deadline_ms: u32,
    name: &'static str,
}

impl Task {
    fn new(deadline_ms: u32, name: &'static str) -> Self {
        Self { deadline_ms, name }
    }
}
