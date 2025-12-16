//! Transition table for FSM validation.

use super::state::NodeState;
use super::event::StateEvent;
use std::collections::BTreeSet;

/// Valid transitions lookup table.
pub struct TransitionTable {
    valid: BTreeSet<(NodeState, std::mem::Discriminant<StateEvent>)>,
}

impl TransitionTable {
    pub fn new() -> Self {
        let mut valid = BTreeSet::new();
        
        // Define all valid transitions
        valid.insert((NodeState::Bootstrap, std::mem::discriminant(&StateEvent::PeerDiscovered { node_id: unsafe { std::mem::zeroed() } })));
        valid.insert((NodeState::Syncing, std::mem::discriminant(&StateEvent::SyncCompleted)));
        valid.insert((NodeState::Syncing, std::mem::discriminant(&StateEvent::SecurityViolation { reason: String::new() })));
        valid.insert((NodeState::Active, std::mem::discriminant(&StateEvent::TrustBelowWarn)));
        valid.insert((NodeState::Active, std::mem::discriminant(&StateEvent::TrustCritical)));
        valid.insert((NodeState::Active, std::mem::discriminant(&StateEvent::SecurityViolation { reason: String::new() })));
        valid.insert((NodeState::Degraded, std::mem::discriminant(&StateEvent::TrustRecovered)));
        valid.insert((NodeState::Degraded, std::mem::discriminant(&StateEvent::TrustCritical)));
        valid.insert((NodeState::Degraded, std::mem::discriminant(&StateEvent::SecurityViolation { reason: String::new() })));
        
        Self { valid }
    }

    pub fn is_valid(&self, state: NodeState, event: &StateEvent) -> bool {
        self.valid.contains(&(state, std::mem::discriminant(event)))
    }
}

impl Default for TransitionTable {
    fn default() -> Self {
        Self::new()
    }
}
