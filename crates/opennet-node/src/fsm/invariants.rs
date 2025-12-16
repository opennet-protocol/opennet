//! FSM invariants that must always hold.

use super::state::NodeState;

/// Verify FSM invariants.
pub fn check_invariants(state: NodeState) -> bool {
    // Invariant 1: State must be one of the defined states
    let valid_state = matches!(
        state,
        NodeState::Bootstrap | NodeState::Syncing | NodeState::Active | 
        NodeState::Degraded | NodeState::Quarantined
    );
    
    valid_state
}

/// Verify transition invariant.
pub fn check_transition_invariant(from: NodeState, to: NodeState) -> bool {
    // Invariant: Cannot transition out of Quarantined
    if from == NodeState::Quarantined {
        return false;
    }
    
    // Invariant: Cannot skip states (no Bootstrap -> Active)
    if from == NodeState::Bootstrap && to == NodeState::Active {
        return false;
    }
    
    true
}
