use rust_data_structures::deque::Deque;

fn main() {
    let values = [4, 2, 12, 3, 8];
    let maximums = sliding_window_maximums(&values, 3);

    assert_eq!(maximums, vec![12, 12, 12]);
    println!("{maximums:?}");
}

fn sliding_window_maximums(values: &[i32], window: usize) -> Vec<i32> {
    let mut indexes = Deque::new();
    let mut maximums = Vec::new();

    for index in 0..values.len() {
        while let Some(&front) = indexes.front() {
            if front + window > index {
                break;
            }
            indexes.pop_front();
        }

        while let Some(&back) = indexes.back() {
            if values[back] >= values[index] {
                break;
            }
            indexes.pop_back();
        }

        indexes.push_back(index);

        if index + 1 >= window {
            let front = indexes.front().copied().expect("window has a maximum");
            maximums.push(values[front]);
        }
    }

    maximums
}
