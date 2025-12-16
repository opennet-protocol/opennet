use opennet_core::NodeId;
use crate::error::Result;

pub struct HandshakeResponder { local_node_id: NodeId }

impl HandshakeResponder {
    pub fn new(node_id: NodeId) -> Self { Self { local_node_id: node_id } }
    pub async fn respond(&self) -> Result<NodeId> { Ok(self.local_node_id) }
}
