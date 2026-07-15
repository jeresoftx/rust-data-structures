use rust_data_structures::skip_list::SkipList;

fn main() {
    let mut index = SkipList::new();

    index.insert(30);
    index.insert(10);
    index.insert(20);

    println!("contiene 20: {}", index.contains(&20));
    println!("contiene 40: {}", index.contains(&40));
    println!("orden: {:?}", index.iter().copied().collect::<Vec<_>>());
}
