use rust_data_structures::vector::Vector;

fn main() {
    let mut values = Vector::new();
    values.push(10);
    values.push(30);
    values.push(50);

    insert_sorted(&mut values, 40);
    insert_sorted(&mut values, 20);

    let ordered = values.iter().copied().collect::<Vec<_>>();
    assert_eq!(ordered, vec![10, 20, 30, 40, 50]);
    println!("{ordered:?}");
}

fn insert_sorted(values: &mut Vector<i32>, value: i32) {
    let mut index = 0;

    while let Some(current) = values.get(index) {
        if *current >= value {
            break;
        }
        index += 1;
    }

    values
        .insert(index, value)
        .expect("index is between 0 and len");
}
