use rust_data_structures::vector::Vector;

#[derive(Debug)]
struct CandidateRoom {
    code: &'static str,
    nightly_price_cents: u32,
}

fn main() {
    let mut candidates = Vector::new();

    candidates.push(CandidateRoom {
        code: "STD-01",
        nightly_price_cents: 14_000,
    });
    candidates.push(CandidateRoom {
        code: "KING-02",
        nightly_price_cents: 21_000,
    });
    candidates.push(CandidateRoom {
        code: "STE-07",
        nightly_price_cents: 32_000,
    });

    let cheapest = candidates
        .iter()
        .min_by_key(|room| room.nightly_price_cents)
        .expect("hay al menos una habitacion candidata");

    println!(
        "habitacion candidata mas barata: {} (${})",
        cheapest.code,
        cheapest.nightly_price_cents / 100
    );
}
