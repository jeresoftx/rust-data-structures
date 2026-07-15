use rust_data_structures::skip_list::SkipList;

fn main() {
    let mut list = SkipList::with_seed(10, 0.5, 77).unwrap();

    for value in [50, 10, 40, 20, 30] {
        assert!(list.insert(value));
    }

    assert!(!list.insert(30));
    assert_eq!(list.remove(&10), Some(10));
    assert_eq!(list.remove(&99), None);

    println!(
        "despues de borrar: {:?}",
        list.iter().copied().collect::<Vec<_>>()
    );
}
