//! Node state definitions.
//!
//! BOOTSTRAP → SYNCING → ACTIVE ↔ DEGRADED → QUARANTINED

/// Node states - a node MUST be in exactly one state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NodeState {
    /// Initial startup, no trusted peers.
    Bootstrap,
    /// Synchronizing trust graph, epochs, revocations.
    Syncing,
    /// Fully operational.
    Active,
    /// Reduced capacity, temporary failures.
    Degraded,
    /// Isolated, revoked or unsafe.
    Quarantined,
}

impl NodeState {
    /// Check if state allows network operations.
    pub fn can_operate(&self) -> bool {
        matches!(self, NodeState::Active | NodeState::Degraded)
    }

    /// Check if state allows accepting connections.
    pub fn can_accept(&self) -> bool {
        matches!(self, NodeState::Active)
    }

    /// Check if state is terminal.
    pub fn is_terminal(&self) -> bool {
        matches!(self, NodeState::Quarantined)
    }
}

impl Default for NodeState {
    fn default() -> Self {
        NodeState::Bootstrap
    }
}

impl std::fmt::Display for NodeState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeState::Bootstrap => write!(f, "BOOTSTRAP"),
            NodeState::Syncing => write!(f, "SYNCING"),
            NodeState::Active => write!(f, "ACTIVE"),
            NodeState::Degraded => write!(f, "DEGRADED"),
            NodeState::Quarantined => write!(f, "QUARANTINED"),
        }
    }
}
