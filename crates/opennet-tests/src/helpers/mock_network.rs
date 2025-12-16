use opennet_core::NodeId;
use std::collections::BTreeMap;

pub struct MockNetwork {
    nodes: BTreeMap<NodeId, MockNode>,
    latency_ms: u64,
}

struct MockNode {
    online: bool,
}

impl MockNetwork {
    pub fn new() -> Self {
        Self { nodes: BTreeMap::new(), latency_ms: 10 }
    }

    pub fn add_node(&mut self, node_id: NodeId) {
        self.nodes.insert(node_id, MockNode { online: true });
    }

    pub fn set_offline(&mut self, node_id: &NodeId) {
        if let Some(node) = self.nodes.get_mut(node_id) {
            node.online = false;
        }
    }

    pub fn set_latency(&mut self, ms: u64) {
        self.latency_ms = ms;
    }
}

impl Default for MockNetwork {
    fn default() -> Self { Self::new() }
}
