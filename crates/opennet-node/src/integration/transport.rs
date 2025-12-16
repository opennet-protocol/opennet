//! Transport integration.

use opennet_core::NodeId;
use opennet_transport::session::SessionManager;

pub struct TransportIntegration {
    sessions: SessionManager,
}

impl TransportIntegration {
    pub fn new() -> Self {
        Self { sessions: SessionManager::new() }
    }

    pub async fn connect(&mut self, _node_id: &NodeId) -> bool {
        true
    }
}

impl Default for TransportIntegration {
    fn default() -> Self { Self::new() }
}
