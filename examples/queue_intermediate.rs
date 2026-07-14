use rust_data_structures::queue::Queue;

fn main() {
    let mut jobs = Queue::new();

    jobs.enqueue("generar docs");
    jobs.enqueue("correr tests");
    jobs.enqueue("publicar reporte");

    while let Some(job) = jobs.dequeue() {
        println!("procesando: {job}");
    }
}
