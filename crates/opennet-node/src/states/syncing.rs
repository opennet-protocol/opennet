//! SYNCING state behavior.

use crate::sync::SyncManager;
use crate::fsm::StateEvent;

/// Syncing state handler.
pub struct SyncingState {
    trust_synced: bool,
    revocations_synced: bool,
    epochs_synced: bool,
}

impl SyncingState {
    pub fn new() -> Self {
        Self {
            trust_synced: false,
            revocations_synced: false,
            epochs_synced: false,
        }
    }

    /// Check if sync is complete.
    pub fn is_complete(&self) -> bool {
        self.trust_synced && self.revocations_synced && self.epochs_synced
    }

    /// Run sync step.
    pub async fn sync_step(&mut self, sync: &mut SyncManager) -> Option<StateEvent> {
        if !self.trust_synced {
            self.trust_synced = sync.sync_trust().await;
        } else if !self.revocations_synced {
            self.revocations_synced = sync.sync_revocations().await;
        } else if !self.epochs_synced {
            self.epochs_synced = sync.sync_epochs().await;
        }

        if self.is_complete() {
            Some(StateEvent::SyncCompleted)
        } else {
            None
        }
    }
}

impl Default for SyncingState {
    fn default() -> Self {
        Self::new()
    }
}
