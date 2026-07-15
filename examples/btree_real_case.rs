use rust_data_structures::btree::BTree;

fn main() {
    let mut database_page_index = BTree::with_min_degree(4).unwrap();

    for page_id in [1004, 1001, 1007, 1003, 1002, 1008, 1006, 1005] {
        database_page_index.insert(page_id);
    }

    println!("indice de paginas:");
    for page_id in database_page_index.iter() {
        println!("page:{page_id}");
    }

    println!(
        "page 1006 localizada: {}",
        database_page_index.contains(&1006)
    );
}
