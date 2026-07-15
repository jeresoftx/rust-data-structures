use rust_data_structures::skip_list::SkipList;

fn main() {
    let mut pending_jobs = SkipList::with_seed(12, 0.5, 123).unwrap();

    for scheduled_at in [1_720_000_300, 1_720_000_100, 1_720_000_200] {
        pending_jobs.insert(scheduled_at);
    }

    let now = 1_720_000_200;
    let ready = pending_jobs
        .iter()
        .copied()
        .take_while(|scheduled_at| *scheduled_at <= now)
        .collect::<Vec<_>>();

    for scheduled_at in ready {
        pending_jobs.remove(&scheduled_at);
        println!("ejecutar job programado en {scheduled_at}");
    }

    println!(
        "pendientes: {:?}",
        pending_jobs.iter().copied().collect::<Vec<_>>()
    );
}
