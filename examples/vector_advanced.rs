use rust_data_structures::vector::Vector;

fn main() {
    let mut values = Vector::with_capacity(2);
    println!("capacidad inicial: {}", values.capacity());

    for value in 0..6 {
        values.push(value);
        println!(
            "push({value}) -> len: {}, capacity: {}",
            values.len(),
            values.capacity()
        );
    }

    values.clear();
    println!(
        "despues de clear -> len: {}, capacity conservada: {}",
        values.len(),
        values.capacity()
    );
}
