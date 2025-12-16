//! FSM events that trigger state transitions.

use opennet_core::NodeId;

/// Events that can trigger state transitions.
#[derive(Debug, Clone)]
pub enum StateEvent {
    /// At least one valid peer discovered.
    PeerDiscovered { node_id: NodeId },
    /// Synchronization completed successfully.
    SyncCompleted,
    /// Trust dropped below warning threshold.
    TrustBelowWarn,
    /// Trust recovered above OK threshold.
    TrustRecovered,
    /// Trust dropped below critical threshold.
    TrustCritical,
    /// Security violation detected.
    SecurityViolation { reason: String },
    /// Epoch expired.
    EpochExpired,
    /// Network timeout.
    NetworkTimeout,
}

impl StateEvent {
    /// Get event priority (higher = more urgent).
    /// SecurityViolation and TrustCritical have highest priority.
    pub fn priority(&self) -> u8 {
        match self {
            StateEvent::SecurityViolation { .. } => 255,
            StateEvent::TrustCritical => 254,
            StateEvent::EpochExpired => 200,
            StateEvent::TrustBelowWarn => 100,
            StateEvent::NetworkTimeout => 50,
            StateEvent::TrustRecovered => 30,
            StateEvent::SyncCompleted => 20,
            StateEvent::PeerDiscovered { .. } => 10,
        }
    }
}

impl PartialEq for StateEvent {
    fn eq(&self, other: &Self) -> bool {
        std::mem::discriminant(self) == std::mem::discriminant(other)
    }
}
