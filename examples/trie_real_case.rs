use rust_data_structures::trie::Trie;

fn main() {
    let mut routes = Trie::new();

    routes.insert("/courses");
    routes.insert("/courses/rust-data-structures");
    routes.insert("/docs");

    let request_path = "/courses/rust";
    println!(
        "rutas que empiezan con /courses: {:?}",
        routes.words_with_prefix("/courses")
    );
    println!(
        "request {request_path} comparte prefijo: {}",
        routes.starts_with("/courses")
    );
}
