use rust_data_structures::deque::Deque;
use std::cell::Cell;
use std::rc::Rc;

#[test]
fn push_and_pop_work_at_both_ends() {
    let mut deque = Deque::new();

    assert!(deque.is_empty());
    assert_eq!(deque.len(), 0);
    assert_eq!(deque.pop_front(), None);
    assert_eq!(deque.pop_back(), None);

    deque.push_back("middle");
    deque.push_front("front");
    deque.push_back("back");

    assert_eq!(deque.front(), Some(&"front"));
    assert_eq!(deque.back(), Some(&"back"));
    assert_eq!(deque.pop_front(), Some("front"));
    assert_eq!(deque.pop_back(), Some("back"));
    assert_eq!(deque.pop_front(), Some("middle"));
    assert_eq!(deque.pop_front(), None);
    assert!(deque.is_empty());
}

#[test]
fn circular_buffer_reuses_slots_from_both_ends() {
    let mut deque = Deque::with_capacity(4);

    deque.push_back(1);
    deque.push_back(2);
    deque.push_back(3);
    assert_eq!(deque.pop_front(), Some(1));
    assert_eq!(deque.pop_front(), Some(2));

    deque.push_back(4);
    deque.push_front(0);
    deque.push_back(5);

    assert_eq!(deque.capacity(), 4);
    assert_eq!(deque.iter().copied().collect::<Vec<_>>(), vec![0, 3, 4, 5]);
}

#[test]
fn growth_preserves_order_after_wraparound() {
    let mut deque = Deque::with_capacity(3);

    deque.push_back("b");
    deque.push_back("c");
    deque.push_back("d");
    assert_eq!(deque.pop_front(), Some("b"));
    deque.push_back("e");
    deque.push_front("a");

    assert!(deque.capacity() >= 6);
    assert_eq!(
        deque.iter().copied().collect::<Vec<_>>(),
        vec!["a", "c", "d", "e"]
    );
    assert_eq!(deque.pop_front(), Some("a"));
    assert_eq!(deque.pop_back(), Some("e"));
    assert_eq!(deque.pop_front(), Some("c"));
    assert_eq!(deque.pop_back(), Some("d"));
}

#[test]
fn get_reads_logical_indexes_without_exposing_physical_layout() {
    let mut deque = Deque::with_capacity(4);

    deque.push_back("A");
    deque.push_back("B");
    deque.push_back("C");
    assert_eq!(deque.pop_front(), Some("A"));
    deque.push_back("D");
    deque.push_front("Z");

    assert_eq!(deque.get(0), Some(&"Z"));
    assert_eq!(deque.get(1), Some(&"B"));
    assert_eq!(deque.get(2), Some(&"C"));
    assert_eq!(deque.get(3), Some(&"D"));
    assert_eq!(deque.get(4), None);
}

#[test]
fn clear_removes_values_but_keeps_capacity() {
    let mut deque = Deque::with_capacity(4);

    deque.push_front(10);
    deque.push_back(20);
    deque.clear();

    assert!(deque.is_empty());
    assert_eq!(deque.capacity(), 4);
    assert_eq!(deque.front(), None);
    assert_eq!(deque.back(), None);
}

#[test]
fn pops_transfer_ownership_and_clear_drops_remaining_values() {
    let drops = Rc::new(Cell::new(0));
    let mut deque = Deque::new();

    deque.push_back(DropCounter::new(Rc::clone(&drops)));
    deque.push_front(DropCounter::new(Rc::clone(&drops)));
    deque.push_back(DropCounter::new(Rc::clone(&drops)));

    let popped = deque.pop_back();
    assert_eq!(drops.get(), 0);

    drop(popped);
    assert_eq!(drops.get(), 1);

    deque.clear();
    assert_eq!(drops.get(), 3);
}

struct DropCounter {
    drops: Rc<Cell<usize>>,
}

impl DropCounter {
    fn new(drops: Rc<Cell<usize>>) -> Self {
        Self { drops }
    }
}

impl Drop for DropCounter {
    fn drop(&mut self) {
        self.drops.set(self.drops.get() + 1);
    }
}
