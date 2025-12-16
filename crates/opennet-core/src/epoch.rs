//! Epoch - Bounded validity period for key pairs.
//!
//! RFC: OpenNet Core Protocol §6
//! RFC: Time & Epoch Semantics RFC

use serde::{Deserialize, Serialize};
use crate::error::{CoreError, Result};
use crate::constants::MAX_EPOCH_DURATION;

/// Epoch identifier (monotonically increasing).
pub type EpochId = u64;

/// Epoch represents a bounded validity period for a key pair.
///
/// Each epoch contains:
/// - Monotonically increasing ID
/// - Start timestamp
/// - Maximum duration
/// - Hash of associated key material
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Epoch {
    /// Epoch number (must strictly increase).
    pub id: EpochId,
    /// Epoch start timestamp (seconds since UNIX epoch).
    pub start_time: u64,
    /// Maximum validity duration in seconds.
    pub max_duration: u64,
    /// SHA-256 hash of key material.
    pub key_hash: [u8; 32],
}

impl Epoch {
    /// Create a new epoch.
    pub fn new(id: EpochId, start_time: u64, key_hash: [u8; 32]) -> Self {
        Self {
            id,
            start_time,
            max_duration: MAX_EPOCH_DURATION,
            key_hash,
        }
    }

    /// Check if epoch is valid at given timestamp.
    pub fn is_valid_at(&self, timestamp: u64) -> bool {
        timestamp >= self.start_time && timestamp < self.end_time()
    }

    /// Get epoch end time.
    pub fn end_time(&self) -> u64 {
        self.start_time.saturating_add(self.max_duration)
    }

    /// Check if this epoch is newer than another.
    pub fn is_newer_than(&self, other: &Epoch) -> bool {
        self.id > other.id
    }

    /// Validate epoch transition (old -> new).
    ///
    /// Returns error if transition is invalid.
    pub fn validate_transition(old: &Epoch, new: &Epoch) -> Result<()> {
        // Epoch ID must strictly increase
        if new.id <= old.id {
            return Err(CoreError::InvalidEpoch(
                "epoch id must strictly increase".into()
            ));
        }

        // New epoch must start before old epoch ends
        if new.start_time > old.end_time() {
            return Err(CoreError::InvalidEpoch(
                "gap between epochs not allowed".into()
            ));
        }

        Ok(())
    }
}

impl Ord for Epoch {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for Epoch {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
