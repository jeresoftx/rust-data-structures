use rust_data_structures::linked_list::LinkedList;

fn main() {
    let mut retry_queue = LinkedList::new();

    retry_queue.push_back(Job::new("sync-calendar", 1));
    retry_queue.push_back(Job::new("send-webhook", 2));
    retry_queue.push_back(Job::new("rebuild-index", 1));

    while let Some(job) = retry_queue.pop_front() {
        println!("ejecutando {} (intento {})", job.name, job.attempt);
    }
}

struct Job {
    name: &'static str,
    attempt: u8,
}

impl Job {
    fn new(name: &'static str, attempt: u8) -> Self {
        Self { name, attempt }
    }
}
