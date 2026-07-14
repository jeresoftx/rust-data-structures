use rust_data_structures::deque::Deque;

fn main() {
    let mut work = Deque::new();

    work.push_back("sincronizar docs");
    work.push_back("correr tests");
    work.push_front("incidente urgente");

    while let Some(job) = work.pop_front() {
        println!("procesando: {job}");
    }
}
