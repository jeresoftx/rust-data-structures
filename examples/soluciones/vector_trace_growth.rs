use rust_data_structures::vector::Vector;

fn main() {
    let mut values = Vector::new();
    let mut trace = Vec::new();

    for value in [10, 20, 30, 40, 50] {
        values.push(value);
        trace.push((values.len(), values.capacity()));
    }

    assert_eq!(trace, vec![(1, 1), (2, 2), (3, 4), (4, 4), (5, 8)]);
    println!("{trace:?}");
}
