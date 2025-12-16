use opennet_core::NodeId;
use std::collections::BTreeMap;

pub struct PeerManager {
    peers: BTreeMap<NodeId, PeerInfo>,
}

#[derive(Debug)]
pub struct PeerInfo {
    pub endpoint: String,
    pub trust: f64,
    pub last_seen: u64,
}

impl PeerManager {
    pub fn new() -> Self { Self { peers: BTreeMap::new() } }
    
    pub fn add_peer(&mut self, node_id: NodeId, info: PeerInfo) {
        self.peers.insert(node_id, info);
    }
    
    pub fn get_peer(&self, node_id: &NodeId) -> Option<&PeerInfo> {
        self.peers.get(node_id)
    }
    
    pub async fn discover_one(&mut self) -> Option<NodeId> {
        self.peers.keys().next().copied()
    }

    pub fn peer_count(&self) -> usize {
        self.peers.len()
    }
}

impl Default for PeerManager {
    fn default() -> Self { Self::new() }
}
