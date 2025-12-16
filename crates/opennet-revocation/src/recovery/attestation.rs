use opennet_core::NodeId;
use opennet_core::types::Signature;

#[derive(Debug, Clone)]
pub struct PeerAttestation {
    pub attester: NodeId,
    pub signature: Signature,
}
