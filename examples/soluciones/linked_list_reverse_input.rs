use rust_data_structures::linked_list::LinkedList;

fn main() {
    let reversed = reverse_with_push_front([1, 2, 3, 4]);
    let values = reversed.iter().copied().collect::<Vec<_>>();

    assert_eq!(values, vec![4, 3, 2, 1]);
    println!("{values:?}");
}

fn reverse_with_push_front(values: impl IntoIterator<Item = i32>) -> LinkedList<i32> {
    let mut list = LinkedList::new();

    for value in values {
        list.push_front(value);
    }

    list
}
