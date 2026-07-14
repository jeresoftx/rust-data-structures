use rust_data_structures::linked_list::LinkedList;

fn main() {
    let mut list = LinkedList::new();

    for value in 1..=5 {
        list.push_back(value);
    }

    let removed = list.remove(2).expect("index 2 exists");
    let remaining = list.iter().copied().collect::<Vec<_>>();

    println!("removido: {removed}");
    println!("restantes: {remaining:?}");
}
