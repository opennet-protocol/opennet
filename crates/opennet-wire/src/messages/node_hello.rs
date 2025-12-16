//! NodeHello message - initial handshake.

use opennet_core::{NodeId, Epoch};
use opennet_core::types::{PublicKey, Timestamp};
use serde::{Deserialize, Serialize};

/// NodeHello message sent to initiate connection.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeHello {
    /// Sender's NodeId.
    pub node_id: NodeId,
    /// Current epoch.
    pub epoch: Epoch,
    /// Current public key.
    pub public_key: PublicKey,
    /// Current timestamp.
    pub timestamp: Timestamp,
    /// Protocol version.
    pub version: u16,
    /// Supported features (bitmask).
    pub features: u64,
}

impl NodeHello {
    /// Create new NodeHello.
    pub fn new(
        node_id: NodeId,
        epoch: Epoch,
        public_key: PublicKey,
        timestamp: Timestamp,
    ) -> Self {
        Self {
            node_id,
            epoch,
            public_key,
            timestamp,
            version: opennet_core::PROTOCOL_VERSION,
            features: 0,
        }
    }
}
