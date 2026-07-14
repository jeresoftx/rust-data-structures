use rust_data_structures::linked_list::LinkedList;

fn main() {
    let mut pages = LinkedList::new();

    pages.push_front("capitulo-vector");
    pages.push_front("prefacio");

    println!("primera pagina: {}", pages.front().unwrap());
    println!("ultima pagina: {}", pages.back().unwrap());
    println!("paginas: {}", pages.len());
}
