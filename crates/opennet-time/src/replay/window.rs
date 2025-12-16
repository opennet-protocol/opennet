//! Replay window tracking.

use opennet_core::types::Timestamp;
use crate::error::{TimeError, Result};

/// Replay window for message validation.
pub struct ReplayWindow {
    window_secs: u64,
}

impl ReplayWindow {
    pub fn new(window_secs: u64) -> Self {
        Self { window_secs }
    }

    /// Check if message timestamp is within window.
    pub fn is_valid(&self, msg_timestamp: Timestamp, current: Timestamp) -> Result<()> {
        let diff = if current.as_secs() > msg_timestamp.as_secs() {
            current.as_secs() - msg_timestamp.as_secs()
        } else {
            msg_timestamp.as_secs() - current.as_secs()
        };

        if diff > self.window_secs {
            return Err(TimeError::InvalidTimestamp);
        }

        Ok(())
    }
}

impl Default for ReplayWindow {
    fn default() -> Self {
        Self::new(120) // 2 minutes
    }
}
