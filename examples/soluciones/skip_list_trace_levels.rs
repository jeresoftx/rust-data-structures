use rust_data_structures::skip_list::SkipList;

fn main() {
    let mut list = SkipList::with_seed(8, 0.5, 2026).unwrap();

    for value in 0..16 {
        list.insert(value);
    }

    let histogram = list.level_histogram();
    assert_eq!(histogram.iter().sum::<usize>(), list.len());
    assert!(list.current_level() <= list.max_level());

    println!("histograma de niveles: {histogram:?}");
}
