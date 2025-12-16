//! Monotonic clock abstraction.

use opennet_core::types::Timestamp;

/// Monotonic clock trait.
pub trait MonotonicClock: Send + Sync {
    /// Get current timestamp.
    fn now(&self) -> Timestamp;
}

/// System clock implementation (for production).
pub struct SystemClock;

impl MonotonicClock for SystemClock {
    fn now(&self) -> Timestamp {
        let duration = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default();
        Timestamp::new(duration.as_secs())
    }
}

/// Mock clock for testing.
pub struct MockClock {
    current: std::sync::atomic::AtomicU64,
}

impl MockClock {
    pub fn new(initial: u64) -> Self {
        Self { current: std::sync::atomic::AtomicU64::new(initial) }
    }
    
    pub fn advance(&self, secs: u64) {
        self.current.fetch_add(secs, std::sync::atomic::Ordering::SeqCst);
    }
    
    pub fn set(&self, secs: u64) {
        self.current.store(secs, std::sync::atomic::Ordering::SeqCst);
    }
}

impl MonotonicClock for MockClock {
    fn now(&self) -> Timestamp {
        Timestamp::new(self.current.load(std::sync::atomic::Ordering::SeqCst))
    }
}
