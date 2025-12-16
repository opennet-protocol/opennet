//! Deterministic event queue.

use crate::fsm::StateEvent;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

/// Event with priority for queue ordering.
struct PrioritizedEvent {
    event: StateEvent,
    sequence: u64,
}

impl PartialEq for PrioritizedEvent {
    fn eq(&self, other: &Self) -> bool {
        self.event.priority() == other.event.priority() && self.sequence == other.sequence
    }
}

impl Eq for PrioritizedEvent {}

impl PartialOrd for PrioritizedEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PrioritizedEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        // Higher priority first, then lower sequence (FIFO within priority)
        match other.event.priority().cmp(&self.event.priority()) {
            Ordering::Equal => self.sequence.cmp(&other.sequence),
            other => other,
        }
    }
}

/// Deterministic priority event queue.
pub struct EventQueue {
    heap: BinaryHeap<PrioritizedEvent>,
    sequence: u64,
}

impl EventQueue {
    pub fn new() -> Self {
        Self { heap: BinaryHeap::new(), sequence: 0 }
    }

    /// Push event with automatic sequencing.
    pub fn push(&mut self, event: StateEvent) {
        self.sequence += 1;
        self.heap.push(PrioritizedEvent { event, sequence: self.sequence });
    }

    /// Pop highest priority event.
    pub fn pop(&mut self) -> Option<StateEvent> {
        self.heap.pop().map(|pe| pe.event)
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    /// Get queue length.
    pub fn len(&self) -> usize {
        self.heap.len()
    }
}

impl Default for EventQueue {
    fn default() -> Self {
        Self::new()
    }
}
