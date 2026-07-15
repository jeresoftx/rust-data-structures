use rust_data_structures::skip_list::SkipList;

fn main() {
    let mut timestamps = SkipList::new();

    for timestamp in [10, 40, 20, 50, 30] {
        timestamps.insert(timestamp);
    }

    let expired = timestamps
        .iter()
        .copied()
        .take_while(|timestamp| *timestamp <= 30)
        .collect::<Vec<_>>();

    for timestamp in expired {
        timestamps.remove(&timestamp);
    }

    assert_eq!(timestamps.iter().copied().collect::<Vec<_>>(), vec![40, 50]);
    println!("ventana expirada removida");
}
