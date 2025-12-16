use opennet_node::fsm::{NodeState, StateEvent, transition};

pub fn test_valid_transitions() -> bool {
    // BOOTSTRAP -> SYNCING
    let result = transition(NodeState::Bootstrap, &StateEvent::PeerDiscovered { 
        node_id: opennet_core::NodeId::from_bytes([0u8; 32])
    });
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), NodeState::Syncing);
    
    // SYNCING -> ACTIVE
    let result = transition(NodeState::Syncing, &StateEvent::SyncCompleted);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), NodeState::Active);
    
    true
}

pub fn test_invalid_transitions() -> bool {
    // BOOTSTRAP -> ACTIVE (invalid)
    let result = transition(NodeState::Bootstrap, &StateEvent::SyncCompleted);
    assert!(result.is_err());
    
    // QUARANTINED -> anything (invalid)
    let result = transition(NodeState::Quarantined, &StateEvent::TrustRecovered);
    assert!(result.is_err());
    
    true
}

pub fn test_event_priority() -> bool {
    let security = StateEvent::SecurityViolation { reason: "test".into() };
    let trust = StateEvent::TrustBelowWarn;
    
    assert!(security.priority() > trust.priority());
    true
}
