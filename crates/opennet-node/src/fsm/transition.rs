//! State transition logic.

use super::state::NodeState;
use super::event::StateEvent;
use crate::error::{NodeError, Result};

/// Represents a state transition.
#[derive(Debug, Clone)]
pub struct Transition {
    pub from: NodeState,
    pub to: NodeState,
    pub event: StateEvent,
}

/// Execute state transition.
///
/// CRITICAL: Any transition not defined MUST panic.
pub fn transition(current: NodeState, event: &StateEvent) -> Result<NodeState> {
    let next = match (current, event) {
        // BOOTSTRAP transitions
        (NodeState::Bootstrap, StateEvent::PeerDiscovered { .. }) => NodeState::Syncing,
        
        // SYNCING transitions
        (NodeState::Syncing, StateEvent::SyncCompleted) => NodeState::Active,
        (NodeState::Syncing, StateEvent::SecurityViolation { .. }) => NodeState::Quarantined,
        
        // ACTIVE transitions
        (NodeState::Active, StateEvent::TrustBelowWarn) => NodeState::Degraded,
        (NodeState::Active, StateEvent::TrustCritical) => NodeState::Quarantined,
        (NodeState::Active, StateEvent::SecurityViolation { .. }) => NodeState::Quarantined,
        
        // DEGRADED transitions
        (NodeState::Degraded, StateEvent::TrustRecovered) => NodeState::Active,
        (NodeState::Degraded, StateEvent::TrustCritical) => NodeState::Quarantined,
        (NodeState::Degraded, StateEvent::SecurityViolation { .. }) => NodeState::Quarantined,
        
        // QUARANTINED - terminal state, no transitions out
        (NodeState::Quarantined, _) => {
            return Err(NodeError::InvalidTransition {
                from: "QUARANTINED".into(),
                to: "any".into(),
            });
        }
        
        // Any undefined transition is a protocol violation
        (from, _) => {
            return Err(NodeError::InvalidTransition {
                from: from.to_string(),
                to: "undefined".into(),
            });
        }
    };
    
    Ok(next)
}
