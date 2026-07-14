use rust_data_structures::heap::Heap;

fn main() {
    let mut priorities = Heap::new();

    priorities.push(30);
    priorities.push(10);
    priorities.push(50);

    println!("prioridad maxima: {}", priorities.peek().unwrap());
    println!("atendido: {}", priorities.pop().unwrap());
}
