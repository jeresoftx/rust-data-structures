use rust_data_structures::queue::Queue;
use std::cell::Cell;
use std::rc::Rc;

#[test]
fn enqueue_and_dequeue_follow_fifo_order() {
    let mut queue = Queue::new();

    assert!(queue.is_empty());
    assert_eq!(queue.len(), 0);
    assert_eq!(queue.dequeue(), None);

    queue.enqueue("first");
    queue.enqueue("second");
    queue.enqueue("third");

    assert_eq!(queue.front(), Some(&"first"));
    assert_eq!(queue.back(), Some(&"third"));
    assert_eq!(queue.dequeue(), Some("first"));
    assert_eq!(queue.dequeue(), Some("second"));
    assert_eq!(queue.dequeue(), Some("third"));
    assert_eq!(queue.dequeue(), None);
    assert!(queue.is_empty());
}

#[test]
fn circular_buffer_reuses_front_slots_after_dequeue() {
    let mut queue = Queue::with_capacity(3);

    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(3);

    assert_eq!(queue.dequeue(), Some(1));
    assert_eq!(queue.dequeue(), Some(2));

    queue.enqueue(4);
    queue.enqueue(5);

    assert_eq!(queue.capacity(), 3);
    assert_eq!(queue.iter().copied().collect::<Vec<_>>(), vec![3, 4, 5]);
}

#[test]
fn growth_preserves_fifo_order_across_wraparound() {
    let mut queue = Queue::with_capacity(3);

    queue.enqueue("a");
    queue.enqueue("b");
    queue.enqueue("c");
    assert_eq!(queue.dequeue(), Some("a"));
    queue.enqueue("d");
    queue.enqueue("e");

    assert!(queue.capacity() >= 6);
    assert_eq!(
        queue.iter().copied().collect::<Vec<_>>(),
        vec!["b", "c", "d", "e"]
    );
    assert_eq!(queue.dequeue(), Some("b"));
    assert_eq!(queue.dequeue(), Some("c"));
    assert_eq!(queue.dequeue(), Some("d"));
    assert_eq!(queue.dequeue(), Some("e"));
}

#[test]
fn clear_removes_values_but_keeps_capacity() {
    let mut queue = Queue::with_capacity(4);

    queue.enqueue(10);
    queue.enqueue(20);
    queue.clear();

    assert!(queue.is_empty());
    assert_eq!(queue.capacity(), 4);
    assert_eq!(queue.front(), None);
    assert_eq!(queue.back(), None);
}

#[test]
fn dequeue_transfers_ownership_and_clear_drops_remaining_values() {
    let drops = Rc::new(Cell::new(0));
    let mut queue = Queue::new();

    queue.enqueue(DropCounter::new(Rc::clone(&drops)));
    queue.enqueue(DropCounter::new(Rc::clone(&drops)));
    queue.enqueue(DropCounter::new(Rc::clone(&drops)));

    let dequeued = queue.dequeue();
    assert_eq!(drops.get(), 0);

    drop(dequeued);
    assert_eq!(drops.get(), 1);

    queue.clear();
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
