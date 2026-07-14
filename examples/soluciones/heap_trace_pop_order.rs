use rust_data_structures::heap::Heap;

fn main() {
    let mut heap = Heap::new();
    heap.push(4);
    heap.push(10);
    heap.push(7);
    heap.push(1);

    let mut popped = Vec::new();
    while let Some(value) = heap.pop() {
        popped.push(value);
    }

    assert_eq!(popped, vec![10, 7, 4, 1]);
    println!("{popped:?}");
}
