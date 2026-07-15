use rust_data_structures::skip_list::SkipList;

fn main() {
    let mut list = SkipList::with_seed(8, 0.5, 2026).unwrap();

    for value in [12, 4, 18, 1, 9, 15, 20, 7] {
        list.insert(value);
    }

    println!("len: {}", list.len());
    println!("nivel actual: {}", list.current_level());
    println!("histograma de alturas: {:?}", list.level_histogram());
}
