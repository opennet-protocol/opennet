//! Epoch expiry tracking.

use opennet_core::EpochId;
use opennet_core::types::Timestamp;
use std::collections::BTreeMap;

/// Tracks epoch expiry times.
pub struct EpochExpiry {
    expiry_times: BTreeMap<EpochId, u64>,
}

impl EpochExpiry {
    pub fn new() -> Self {
        Self { expiry_times: BTreeMap::new() }
    }

    /// Register epoch expiry.
    pub fn register(&mut self, epoch_id: EpochId, expiry_time: u64) {
        self.expiry_times.insert(epoch_id, expiry_time);
    }

    /// Check if epoch is expired.
    pub fn is_expired(&self, epoch_id: EpochId, at: Timestamp) -> bool {
        self.expiry_times.get(&epoch_id)
            .map(|&expiry| at.as_secs() >= expiry)
            .unwrap_or(false)
    }

    /// Get expired epochs at timestamp.
    pub fn get_expired(&self, at: Timestamp) -> Vec<EpochId> {
        self.expiry_times.iter()
            .filter(|(_, &expiry)| at.as_secs() >= expiry)
            .map(|(&id, _)| id)
            .collect()
    }
}

impl Default for EpochExpiry {
    fn default() -> Self {
        Self::new()
    }
}
