use opennet_core::NodeId;
use opennet_core::types::{PublicKey, Timestamp};

#[derive(Debug, Clone)]
pub struct RecoveryRequest {
    pub node_id: NodeId,
    pub revoked_epoch: u64,
    pub new_public_key: PublicKey,
    pub timestamp: Timestamp,
}
