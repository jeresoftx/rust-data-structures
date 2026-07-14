use rust_data_structures::queue::Queue;

fn main() {
    let mut queue = Queue::new();
    queue.enqueue(Task::new("docs", 2));
    queue.enqueue(Task::new("tests", 1));

    let completed = run_round_robin(queue);

    assert_eq!(completed, vec!["tests", "docs"]);
    println!("{completed:?}");
}

fn run_round_robin(mut queue: Queue<Task>) -> Vec<&'static str> {
    let mut completed = Vec::new();

    while let Some(mut task) = queue.dequeue() {
        task.remaining -= 1;

        if task.remaining == 0 {
            completed.push(task.name);
        } else {
            queue.enqueue(task);
        }
    }

    completed
}

struct Task {
    name: &'static str,
    remaining: u8,
}

impl Task {
    fn new(name: &'static str, remaining: u8) -> Self {
        Self { name, remaining }
    }
}
