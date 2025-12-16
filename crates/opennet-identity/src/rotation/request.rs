//! Key rotation request.

use opennet_core::NodeId;
use opennet_core::types::{PublicKey, Signature, Timestamp};
use serde::{Deserialize, Serialize};

/// Key rotation request.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RotationRequest {
    /// Node requesting rotation.
    pub node_id: NodeId,
    /// Current epoch ID.
    pub current_epoch: u64,
    /// New epoch ID (must be current + 1).
    pub new_epoch: u64,
    /// New public key.
    pub new_public_key: PublicKey,
    /// Request timestamp.
    pub timestamp: Timestamp,
    /// Signature with OLD key (proves ownership).
    pub old_key_signature: Signature,
    /// Signature with NEW key (proves possession).
    pub new_key_signature: Signature,
}

impl RotationRequest {
    /// Create signing bytes for old key signature.
    pub fn old_key_signing_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(self.node_id.as_bytes());
        bytes.extend_from_slice(&self.current_epoch.to_be_bytes());
        bytes.extend_from_slice(&self.new_epoch.to_be_bytes());
        bytes.extend_from_slice(self.new_public_key.as_bytes());
        bytes.extend_from_slice(&self.timestamp.as_secs().to_be_bytes());
        bytes
    }

    /// Create signing bytes for new key signature.
    pub fn new_key_signing_bytes(&self) -> Vec<u8> {
        // Same content, signed with new key
        self.old_key_signing_bytes()
    }
}
