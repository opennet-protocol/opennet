use opennet_core::NodeId;
use crate::error::Result;

pub struct HandshakeInitiator { local_node_id: NodeId }

impl HandshakeInitiator {
    pub fn new(node_id: NodeId) -> Self { Self { local_node_id: node_id } }
    pub async fn initiate(&self, _remote: &NodeId) -> Result<()> { Ok(()) }
}
