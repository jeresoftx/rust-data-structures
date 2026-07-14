use rust_data_structures::queue::Queue;

fn main() {
    let mut queue = Queue::new();
    queue.enqueue(Job::new("index", true));
    queue.enqueue(Job::new("email", false));
    queue.enqueue(Job::new("report", true));

    let ready = drain_ready(queue);

    assert_eq!(ready, vec!["index", "report"]);
    println!("{ready:?}");
}

fn drain_ready(mut queue: Queue<Job>) -> Vec<&'static str> {
    let mut ready = Vec::new();

    while let Some(job) = queue.dequeue() {
        if job.ready {
            ready.push(job.name);
        }
    }

    ready
}

struct Job {
    name: &'static str,
    ready: bool,
}

impl Job {
    fn new(name: &'static str, ready: bool) -> Self {
        Self { name, ready }
    }
}
