use rust_data_structures::bloom_filter::{BloomFilter, BloomFilterError};

#[test]
fn inserted_values_are_never_reported_missing() {
    let mut filter = BloomFilter::new(128, 3).unwrap();

    for value in ["rust", "data", "structures"] {
        filter.insert(&value);
    }

    assert!(filter.might_contain(&"rust"));
    assert!(filter.might_contain(&"data"));
    assert!(filter.might_contain(&"structures"));
    assert_eq!(filter.inserted_count(), 3);
}

#[test]
fn controlled_missing_values_are_definitely_absent() {
    let mut filter = BloomFilter::new(256, 4).unwrap();

    for value in ["alpha", "beta", "gamma"] {
        filter.insert(&value);
    }

    assert!(!filter.might_contain(&"delta"));
    assert!(!filter.might_contain(&"epsilon"));
}

#[test]
fn parameter_validation_rejects_empty_bit_array_and_zero_hashes() {
    assert!(matches!(
        BloomFilter::new(0, 3),
        Err(BloomFilterError::InvalidBitCount)
    ));
    assert!(matches!(
        BloomFilter::new(128, 0),
        Err(BloomFilterError::InvalidHashCount)
    ));
}

#[test]
fn recommended_sizing_uses_expected_items_and_false_positive_rate() {
    let filter = BloomFilter::with_estimated_items(100, 0.01).unwrap();

    assert!(filter.bit_count() >= 900);
    assert!(filter.hash_count() >= 6);
    assert_eq!(filter.inserted_count(), 0);
}

#[test]
fn recommended_sizing_rejects_invalid_estimates() {
    assert!(matches!(
        BloomFilter::with_estimated_items(0, 0.01),
        Err(BloomFilterError::InvalidExpectedItems)
    ));
    assert!(matches!(
        BloomFilter::with_estimated_items(10, 0.0),
        Err(BloomFilterError::InvalidFalsePositiveRate)
    ));
    assert!(matches!(
        BloomFilter::with_estimated_items(10, 1.0),
        Err(BloomFilterError::InvalidFalsePositiveRate)
    ));
}

#[test]
fn measured_false_positive_rate_stays_below_controlled_threshold() {
    let mut filter = BloomFilter::with_estimated_items(100, 0.05).unwrap();

    for value in 0..100_u64 {
        filter.insert(&value);
    }

    let false_positives = (10_000_u64..11_000)
        .filter(|value| filter.might_contain(value))
        .count();
    let measured_rate = false_positives as f64 / 1_000.0;

    assert!(measured_rate <= 0.10, "measured_rate={measured_rate}");
}

#[test]
fn clear_resets_bits_and_inserted_count() {
    let mut filter = BloomFilter::new(128, 3).unwrap();

    filter.insert(&"cache-key");
    assert!(filter.might_contain(&"cache-key"));

    filter.clear();

    assert_eq!(filter.inserted_count(), 0);
    assert_eq!(filter.set_bit_count(), 0);
    assert!(!filter.might_contain(&"cache-key"));
}
