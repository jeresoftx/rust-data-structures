use rust_data_structures::heap::MinHeap;

fn main() {
    let top = top_k(&[12, 4, 20, 7, 18, 2], 3);

    assert_eq!(top, vec![20, 18, 12]);
    println!("{top:?}");
}

fn top_k(values: &[i32], k: usize) -> Vec<i32> {
    let mut heap = MinHeap::with_capacity(k);

    for &value in values {
        if heap.len() < k {
            heap.push(value);
        } else if heap.peek().is_some_and(|smallest| value > *smallest) {
            heap.pop();
            heap.push(value);
        }
    }

    let mut top = Vec::new();
    while let Some(value) = heap.pop() {
        top.push(value);
    }

    top.reverse();
    top
}
