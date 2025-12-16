use opennet_core::types::Timestamp;
use std::sync::atomic::{AtomicU64, Ordering};

pub struct MockTime {
    current: AtomicU64,
}

impl MockTime {
    pub fn new(initial: u64) -> Self {
        Self { current: AtomicU64::new(initial) }
    }

    pub fn now(&self) -> Timestamp {
        Timestamp::new(self.current.load(Ordering::SeqCst))
    }

    pub fn advance(&self, secs: u64) {
        self.current.fetch_add(secs, Ordering::SeqCst);
    }

    pub fn set(&self, secs: u64) {
        self.current.store(secs, Ordering::SeqCst);
    }
}
