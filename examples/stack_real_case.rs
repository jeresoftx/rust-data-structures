use rust_data_structures::stack::Stack;

fn main() {
    let mut traversal = Stack::new();
    traversal.push(Route::new("/", 0));
    traversal.push(Route::new("/courses", 1));
    traversal.push(Route::new("/courses/rust-data-structures", 2));

    while let Some(route) = traversal.pop() {
        println!("{}{}", "  ".repeat(route.depth), route.path);
    }
}

struct Route {
    path: &'static str,
    depth: usize,
}

impl Route {
    fn new(path: &'static str, depth: usize) -> Self {
        Self { path, depth }
    }
}
