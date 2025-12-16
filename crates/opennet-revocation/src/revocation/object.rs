use opennet_core::NodeId;
use opennet_core::types::{Signature, Timestamp};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevocationObject {
    pub node_id: NodeId,
    pub revoked_epoch: u64,
    pub reason: u8,
    pub timestamp: Timestamp,
    pub signatures: Vec<QuorumSignature>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuorumSignature {
    pub signer: NodeId,
    pub epoch: u64,
    pub signature: Signature,
}
