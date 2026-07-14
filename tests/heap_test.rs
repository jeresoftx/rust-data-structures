use rust_data_structures::heap::{Heap, MinHeap};
use std::cell::Cell;
use std::cmp::Ordering;
use std::rc::Rc;

#[test]
fn max_heap_pops_values_from_largest_to_smallest() {
    let mut heap = Heap::new();

    assert!(heap.is_empty());
    assert_eq!(heap.len(), 0);
    assert_eq!(heap.peek(), None);
    assert_eq!(heap.pop(), None);

    heap.push(4);
    heap.push(10);
    heap.push(7);
    heap.push(1);

    assert_eq!(heap.peek(), Some(&10));
    assert_eq!(heap.pop(), Some(10));
    assert_eq!(heap.pop(), Some(7));
    assert_eq!(heap.pop(), Some(4));
    assert_eq!(heap.pop(), Some(1));
    assert_eq!(heap.pop(), None);
}

#[test]
fn min_heap_pops_values_from_smallest_to_largest() {
    let mut heap = MinHeap::new();

    heap.push(4);
    heap.push(10);
    heap.push(7);
    heap.push(1);

    assert_eq!(heap.peek(), Some(&1));
    assert_eq!(heap.pop(), Some(1));
    assert_eq!(heap.pop(), Some(4));
    assert_eq!(heap.pop(), Some(7));
    assert_eq!(heap.pop(), Some(10));
}

#[test]
fn duplicates_are_preserved() {
    let mut heap = Heap::new();

    heap.push(5);
    heap.push(5);
    heap.push(3);
    heap.push(5);

    assert_eq!(heap.pop(), Some(5));
    assert_eq!(heap.pop(), Some(5));
    assert_eq!(heap.pop(), Some(5));
    assert_eq!(heap.pop(), Some(3));
}

#[test]
fn heapify_builds_valid_max_heap_from_existing_values() {
    let mut heap = Heap::from_vec(vec![3, 1, 9, 2, 8, 7]);

    assert_eq!(heap.len(), 6);
    assert_eq!(heap.peek(), Some(&9));
    assert_eq!(heap.iter_level_order().copied().max(), Some(9));

    let mut popped = Vec::new();
    while let Some(value) = heap.pop() {
        popped.push(value);
    }

    assert_eq!(popped, vec![9, 8, 7, 3, 2, 1]);
}

#[test]
fn heapify_builds_valid_min_heap_from_existing_values() {
    let mut heap = MinHeap::from_vec(vec![3, 1, 9, 2, 8, 7]);

    assert_eq!(heap.peek(), Some(&1));
    assert_eq!(heap.pop(), Some(1));
    assert_eq!(heap.pop(), Some(2));
    assert_eq!(heap.pop(), Some(3));
}

#[test]
fn clear_removes_values_but_keeps_capacity() {
    let mut heap = Heap::with_capacity(8);

    heap.push(10);
    heap.push(20);
    heap.clear();

    assert!(heap.is_empty());
    assert!(heap.capacity() >= 8);
    assert_eq!(heap.peek(), None);
    assert_eq!(heap.pop(), None);
}

#[test]
fn pop_transfers_ownership_and_clear_drops_remaining_values() {
    let drops = Rc::new(Cell::new(0));
    let mut heap = Heap::new();

    heap.push(PriorityDrop::new(2, Rc::clone(&drops)));
    heap.push(PriorityDrop::new(5, Rc::clone(&drops)));
    heap.push(PriorityDrop::new(1, Rc::clone(&drops)));

    let popped = heap.pop();
    assert_eq!(popped.as_ref().map(|item| item.priority), Some(5));
    assert_eq!(drops.get(), 0);

    drop(popped);
    assert_eq!(drops.get(), 1);

    heap.clear();
    assert_eq!(drops.get(), 3);
}

#[derive(Debug)]
struct PriorityDrop {
    priority: i32,
    drops: Rc<Cell<usize>>,
}

impl PriorityDrop {
    fn new(priority: i32, drops: Rc<Cell<usize>>) -> Self {
        Self { priority, drops }
    }
}

impl PartialEq for PriorityDrop {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl Eq for PriorityDrop {}

impl PartialOrd for PriorityDrop {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PriorityDrop {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl Drop for PriorityDrop {
    fn drop(&mut self) {
        self.drops.set(self.drops.get() + 1);
    }
}
