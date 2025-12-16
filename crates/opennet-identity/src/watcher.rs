//! IdentityWatcher - FSM event producer for identity events.

use opennet_core::NodeId;
use crate::node_identity::NodeIdentity;
use crate::compromise::{CompromiseDetector, CompromiseEvidence};

/// Events produced by IdentityWatcher.
#[derive(Debug, Clone)]
pub enum IdentityEvent {
    /// Key rotation completed successfully.
    RotationCompleted { epoch_id: u64 },
    /// Key compromise detected.
    CompromiseDetected { node_id: NodeId, evidence: CompromiseEvidence },
    /// Identity merge successful.
    MergeCompleted,
    /// Identity split detected (anomaly).
    SplitDetected,
}

/// Watches identity changes and produces FSM events.
pub struct IdentityWatcher {
    compromise_detector: CompromiseDetector,
    pending_events: Vec<IdentityEvent>,
}

impl IdentityWatcher {
    /// Create new watcher.
    pub fn new() -> Self {
        Self {
            compromise_detector: CompromiseDetector::new(),
            pending_events: Vec::new(),
        }
    }

    /// Process identity change.
    pub fn on_identity_change(&mut self, identity: &NodeIdentity) {
        // Check for compromise
        if let Some(evidence) = self.compromise_detector.check_duplicate_signatures(identity.node_id()) {
            self.pending_events.push(IdentityEvent::CompromiseDetected {
                node_id: *identity.node_id(),
                evidence,
            });
        }
    }

    /// Process key rotation.
    pub fn on_rotation(&mut self, new_epoch_id: u64) {
        self.pending_events.push(IdentityEvent::RotationCompleted { epoch_id: new_epoch_id });
    }

    /// Drain pending events.
    pub fn drain_events(&mut self) -> Vec<IdentityEvent> {
        std::mem::take(&mut self.pending_events)
    }
}

impl Default for IdentityWatcher {
    fn default() -> Self {
        Self::new()
    }
}
