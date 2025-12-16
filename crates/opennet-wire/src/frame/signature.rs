//! Signature block for signed frames.

use opennet_core::types::Signature;
use opennet_core::NodeId;

/// Signature block attached to frames.
#[derive(Debug, Clone)]
pub struct SignatureBlock {
    /// Signer's NodeId.
    pub signer: NodeId,
    /// Epoch ID of signing key.
    pub epoch_id: u64,
    /// Ed25519 signature.
    pub signature: Signature,
}

impl SignatureBlock {
    /// Create new signature block.
    pub fn new(signer: NodeId, epoch_id: u64, signature: Signature) -> Self {
        Self { signer, epoch_id, signature }
    }
}
