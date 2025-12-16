//! BOOTSTRAP state behavior.

use crate::peer::PeerManager;
use crate::fsm::StateEvent;

/// Bootstrap state handler.
pub struct BootstrapState {
    discovery_attempts: u32,
}

impl BootstrapState {
    pub fn new() -> Self {
        Self { discovery_attempts: 0 }
    }

    /// Attempt peer discovery.
    pub async fn discover_peers(&mut self, peers: &mut PeerManager) -> Option<StateEvent> {
        self.discovery_attempts += 1;
        
        if let Some(node_id) = peers.discover_one().await {
            return Some(StateEvent::PeerDiscovered { node_id });
        }
        
        None
    }
}

impl Default for BootstrapState {
    fn default() -> Self {
        Self::new()
    }
}
