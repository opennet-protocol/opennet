//! Nonce tracking for replay protection.

use std::collections::BTreeSet;
use crate::error::{TimeError, Result};

/// Tracks seen nonces to prevent replay.
pub struct NonceTracker {
    seen: BTreeSet<u64>,
    max_size: usize,
}

impl NonceTracker {
    pub fn new(max_size: usize) -> Self {
        Self { seen: BTreeSet::new(), max_size }
    }

    /// Check and record a nonce.
    pub fn check_and_record(&mut self, nonce: u64) -> Result<()> {
        if self.seen.contains(&nonce) {
            return Err(TimeError::ReplayDetected(nonce));
        }

        self.seen.insert(nonce);

        // Evict oldest if over limit
        while self.seen.len() > self.max_size {
            if let Some(&first) = self.seen.iter().next() {
                self.seen.remove(&first);
            }
        }

        Ok(())
    }

    /// Clear all tracked nonces.
    pub fn clear(&mut self) {
        self.seen.clear();
    }
}

impl Default for NonceTracker {
    fn default() -> Self {
        Self::new(10000)
    }
}
