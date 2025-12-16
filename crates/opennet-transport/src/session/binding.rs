use opennet_core::NodeId;

/// Session bound to (NodeId, Epoch).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SessionBinding {
    pub node_id: NodeId,
    pub epoch_id: u64,
}

impl SessionBinding {
    pub fn new(node_id: NodeId, epoch_id: u64) -> Self { Self { node_id, epoch_id } }
}
