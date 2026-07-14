use rust_data_structures::vector::Vector;

fn main() {
    let mut priorities = Vector::new();

    priorities.push("documentar invariantes");
    priorities.push("escribir benchmarks");
    priorities
        .insert(1, "agregar tests de borde")
        .expect("insertar dentro de la longitud es valido");

    for (index, task) in priorities.iter().enumerate() {
        println!("{index}: {task}");
    }

    let removed = priorities.remove(0).expect("hay una tarea inicial");
    println!("tarea completada: {removed}");
}
