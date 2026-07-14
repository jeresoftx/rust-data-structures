use rust_data_structures::linked_list::LinkedList;

fn main() {
    let mut queue = LinkedList::new();

    queue.push_back("validar README");
    queue.push_back("revisar doctests");
    queue.push_back("actualizar ROADMAP");

    while let Some(task) = queue.pop_front() {
        println!("procesando: {task}");
    }
}
