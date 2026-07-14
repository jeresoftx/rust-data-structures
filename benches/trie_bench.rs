use rust_data_structures::trie::Trie;
use std::collections::HashSet;
use std::hint::black_box;
use std::time::{Duration, Instant};

const SIZE: usize = 10_000;

fn main() {
    let words = generate_words();

    println!("trie benchmark (manual, std::time::Instant)");
    println!("size: {SIZE}");
    println!("trie lookup: {:?}", bench_trie_lookup(&words));
    println!("hashset lookup: {:?}", bench_hashset_lookup(&words));
    println!("trie prefix search: {:?}", bench_trie_prefix_search(&words));
    println!(
        "sorted vector prefix search: {:?}",
        bench_sorted_vector_prefix_search(&words)
    );
}

fn bench_trie_lookup(words: &[String]) -> Duration {
    let mut trie = Trie::new();
    for word in words {
        trie.insert(word);
    }

    let start = Instant::now();
    for word in words {
        black_box(trie.contains(word));
    }
    start.elapsed()
}

fn bench_hashset_lookup(words: &[String]) -> Duration {
    let set = words.iter().collect::<HashSet<_>>();

    let start = Instant::now();
    for word in words {
        black_box(set.contains(word));
    }
    start.elapsed()
}

fn bench_trie_prefix_search(words: &[String]) -> Duration {
    let mut trie = Trie::new();
    for word in words {
        trie.insert(word);
    }

    let start = Instant::now();
    for prefix in ["course-0", "course-1", "course-9"] {
        black_box(trie.words_with_prefix(prefix));
    }
    start.elapsed()
}

fn bench_sorted_vector_prefix_search(words: &[String]) -> Duration {
    let mut sorted = words.to_vec();
    sorted.sort();

    let start = Instant::now();
    for prefix in ["course-0", "course-1", "course-9"] {
        let matches = sorted
            .iter()
            .filter(|word| word.starts_with(prefix))
            .collect::<Vec<_>>();
        black_box(matches);
    }
    start.elapsed()
}

fn generate_words() -> Vec<String> {
    (0..SIZE).map(|index| format!("course-{index:05}")).collect()
}
