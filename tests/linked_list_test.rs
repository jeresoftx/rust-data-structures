use rust_data_structures::linked_list::LinkedList;
use std::cell::Cell;
use std::rc::Rc;

#[test]
fn push_and_pop_front_follow_lifo_order_at_the_head() {
    let mut list = LinkedList::new();

    assert!(list.is_empty());
    assert_eq!(list.len(), 0);
    assert_eq!(list.pop_front(), None);

    list.push_front("tail");
    list.push_front("middle");
    list.push_front("head");

    assert_eq!(list.len(), 3);
    assert_eq!(list.front(), Some(&"head"));
    assert_eq!(list.back(), Some(&"tail"));
    assert_eq!(list.pop_front(), Some("head"));
    assert_eq!(list.pop_front(), Some("middle"));
    assert_eq!(list.pop_front(), Some("tail"));
    assert_eq!(list.pop_front(), None);
    assert!(list.is_empty());
}

#[test]
fn push_back_and_pop_back_preserve_stack_order_at_the_tail() {
    let mut list = LinkedList::new();

    list.push_back(1);
    list.push_back(2);
    list.push_back(3);

    assert_eq!(list.front(), Some(&1));
    assert_eq!(list.back(), Some(&3));
    assert_eq!(list.pop_back(), Some(3));
    assert_eq!(list.pop_back(), Some(2));
    assert_eq!(list.pop_back(), Some(1));
    assert_eq!(list.pop_back(), None);
}

#[test]
fn remove_handles_head_middle_tail_and_missing_indexes() {
    let mut list = LinkedList::new();

    for value in ["a", "b", "c", "d"] {
        list.push_back(value);
    }

    assert_eq!(list.remove(0), Some("a"));
    assert_eq!(list.remove(1), Some("c"));
    assert_eq!(list.remove(1), Some("d"));
    assert_eq!(list.remove(1), None);
    assert_eq!(list.iter().copied().collect::<Vec<_>>(), vec!["b"]);
}

#[test]
fn iter_and_clear_keep_order_and_drop_values() {
    let drops = Rc::new(Cell::new(0));
    let mut list = LinkedList::new();

    list.push_back(DropCounter::new("front", Rc::clone(&drops)));
    list.push_back(DropCounter::new("middle", Rc::clone(&drops)));
    list.push_back(DropCounter::new("back", Rc::clone(&drops)));

    let labels = list.iter().map(|counter| counter.label).collect::<Vec<_>>();
    assert_eq!(labels, vec!["front", "middle", "back"]);

    let removed = list.remove(1);
    assert_eq!(drops.get(), 0);

    drop(removed);
    assert_eq!(drops.get(), 1);

    list.clear();
    assert_eq!(drops.get(), 3);
    assert!(list.is_empty());
}

#[test]
fn single_element_transitions_update_both_ends() {
    let mut list = LinkedList::new();

    list.push_back(10);
    assert_eq!(list.front(), Some(&10));
    assert_eq!(list.back(), Some(&10));

    assert_eq!(list.pop_back(), Some(10));
    assert!(list.is_empty());
    assert_eq!(list.front(), None);
    assert_eq!(list.back(), None);

    list.push_front(20);
    assert_eq!(list.front(), Some(&20));
    assert_eq!(list.back(), Some(&20));

    assert_eq!(list.pop_front(), Some(20));
    assert!(list.is_empty());
}

struct DropCounter {
    label: &'static str,
    drops: Rc<Cell<usize>>,
}

impl DropCounter {
    fn new(label: &'static str, drops: Rc<Cell<usize>>) -> Self {
        Self { label, drops }
    }
}

impl Drop for DropCounter {
    fn drop(&mut self) {
        self.drops.set(self.drops.get() + 1);
    }
}
