use rust_data_structures::heap::MinHeap;

fn main() {
    let values = [12, 4, 20, 7, 18, 2];
    let top = top_k(values, 3);

    println!("top 3: {top:?}");
}

fn top_k<const N: usize>(values: [i32; N], k: usize) -> Vec<i32> {
    let mut heap = MinHeap::with_capacity(k);

    for value in values {
        if heap.len() < k {
            heap.push(value);
        } else if heap.peek().is_some_and(|smallest| value > *smallest) {
            heap.pop();
            heap.push(value);
        }
    }

    let mut result = Vec::new();
    while let Some(value) = heap.pop() {
        result.push(value);
    }
    result.reverse();
    result
}
