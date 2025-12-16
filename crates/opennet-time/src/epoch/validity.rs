//! Epoch validity checking.

use opennet_core::Epoch;
use opennet_core::types::Timestamp;

/// Epoch validity checker.
pub struct EpochValidity;

impl EpochValidity {
    /// Check if epoch is valid at timestamp.
    pub fn is_valid(epoch: &Epoch, at: Timestamp) -> bool {
        epoch.is_valid_at(at.as_secs())
    }

    /// Check if epoch will expire soon.
    pub fn expires_soon(epoch: &Epoch, at: Timestamp, threshold_secs: u64) -> bool {
        let remaining = epoch.end_time().saturating_sub(at.as_secs());
        remaining <= threshold_secs
    }

    /// Get remaining validity in seconds.
    pub fn remaining_secs(epoch: &Epoch, at: Timestamp) -> u64 {
        epoch.end_time().saturating_sub(at.as_secs())
    }
}
