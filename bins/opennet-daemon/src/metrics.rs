//! Metrics export.

use std::sync::atomic::{AtomicU64, Ordering};

pub struct Metrics {
    pub connections: AtomicU64,
    pub messages_in: AtomicU64,
    pub messages_out: AtomicU64,
}

impl Metrics {
    pub fn new() -> Self {
        Self {
            connections: AtomicU64::new(0),
            messages_in: AtomicU64::new(0),
            messages_out: AtomicU64::new(0),
        }
    }

    pub fn inc_connections(&self) {
        self.connections.fetch_add(1, Ordering::Relaxed);
    }

    pub fn dec_connections(&self) {
        self.connections.fetch_sub(1, Ordering::Relaxed);
    }

    pub fn record_message_in(&self) {
        self.messages_in.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_message_out(&self) {
        self.messages_out.fetch_add(1, Ordering::Relaxed);
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}
