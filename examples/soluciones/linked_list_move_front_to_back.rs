use rust_data_structures::linked_list::LinkedList;

fn main() {
    let mut list = LinkedList::new();
    list.push_back("a");
    list.push_back("b");
    list.push_back("c");

    move_front_to_back(&mut list);

    let values = list.iter().copied().collect::<Vec<_>>();
    assert_eq!(values, vec!["b", "c", "a"]);
    println!("{values:?}");
}

fn move_front_to_back<T>(list: &mut LinkedList<T>) {
    if let Some(value) = list.pop_front() {
        list.push_back(value);
    }
}
