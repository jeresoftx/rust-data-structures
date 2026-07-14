use rust_data_structures::linked_list::LinkedList;

fn main() {
    let mut list = LinkedList::new();
    for value in 1..=8 {
        list.push_back(value);
    }

    remove_every_other(&mut list);

    let values = list.iter().copied().collect::<Vec<_>>();
    assert_eq!(values, vec![1, 3, 5, 7]);
    println!("{values:?}");
}

fn remove_every_other<T>(list: &mut LinkedList<T>) {
    let mut index = 1;

    while index < list.len() {
        list.remove(index);
        index += 1;
    }
}
