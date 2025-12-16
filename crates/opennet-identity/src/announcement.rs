//! Identity announcement messages.

use opennet_core::{NodeId, Epoch};
use opennet_core::types::{PublicKey, Signature, Timestamp};
use serde::{Deserialize, Serialize};

/// Identity announcement broadcast to network.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityAnnouncement {
    /// Node identifier.
    pub node_id: NodeId,
    /// Current epoch.
    pub epoch: Epoch,
    /// Current public key.
    pub public_key: PublicKey,
    /// Announcement timestamp.
    pub timestamp: Timestamp,
    /// Self-signature.
    pub signature: Signature,
}

impl IdentityAnnouncement {
    /// Create announcement bytes for signing.
    pub fn signing_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(self.node_id.as_bytes());
        bytes.extend_from_slice(&self.epoch.id.to_be_bytes());
        bytes.extend_from_slice(self.public_key.as_bytes());
        bytes.extend_from_slice(&self.timestamp.as_secs().to_be_bytes());
        bytes
    }
}
