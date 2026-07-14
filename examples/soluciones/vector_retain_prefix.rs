use rust_data_structures::vector::Vector;

fn main() {
    let mut values = Vector::new();

    for value in ["api", "api/v1", "api/v2", "admin", "assets"] {
        values.push(value);
    }

    retain_prefix(&mut values, "api");

    let retained = values.iter().copied().collect::<Vec<_>>();
    assert_eq!(retained, vec!["api", "api/v1", "api/v2"]);
    println!("{retained:?}");
}

fn retain_prefix(values: &mut Vector<&str>, prefix: &str) {
    let mut index = 0;

    while index < values.len() {
        let keep = values
            .get(index)
            .expect("index is inside len")
            .starts_with(prefix);

        if keep {
            index += 1;
        } else {
            values.remove(index);
        }
    }
}
