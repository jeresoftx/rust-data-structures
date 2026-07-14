use rust_data_structures::queue::Queue;

fn main() {
    let mut requests = Queue::new();

    requests.enqueue(Request::new("/courses", 10));
    requests.enqueue(Request::new("/courses/rust-data-structures", 12));
    requests.enqueue(Request::new("/health", 1));

    while let Some(request) = requests.dequeue() {
        println!("{} ({} ms)", request.path, request.estimated_ms);
    }
}

struct Request {
    path: &'static str,
    estimated_ms: u32,
}

impl Request {
    fn new(path: &'static str, estimated_ms: u32) -> Self {
        Self { path, estimated_ms }
    }
}
