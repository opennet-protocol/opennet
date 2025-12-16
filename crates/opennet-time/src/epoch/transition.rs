//! Epoch transition handling.

use opennet_core::EpochId;

/// Represents an epoch transition.
#[derive(Debug, Clone)]
pub struct EpochTransition {
    /// Previous epoch ID.
    pub from_epoch: EpochId,
    /// New epoch ID.
    pub to_epoch: EpochId,
    /// Transition timestamp.
    pub timestamp: u64,
}

impl EpochTransition {
    pub fn new(from: EpochId, to: EpochId, timestamp: u64) -> Self {
        Self { from_epoch: from, to_epoch: to, timestamp }
    }

    /// Validate transition is sequential.
    pub fn is_valid(&self) -> bool {
        self.to_epoch == self.from_epoch + 1
    }
}
