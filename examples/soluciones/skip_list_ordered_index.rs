use rust_data_structures::skip_list::SkipList;

fn main() {
    let mut offsets = SkipList::new();

    for offset in [400, 100, 300, 200] {
        assert!(offsets.insert(offset));
    }

    assert_eq!(
        offsets.iter().copied().collect::<Vec<_>>(),
        vec![100, 200, 300, 400]
    );
    assert!(offsets.contains(&300));

    println!("indice ordenado listo");
}
