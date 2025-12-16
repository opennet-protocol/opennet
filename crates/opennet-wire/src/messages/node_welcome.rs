//! NodeWelcome message - handshake response.

use opennet_core::{NodeId, Epoch};
use opennet_core::types::{PublicKey, Timestamp};
use serde::{Deserialize, Serialize};

/// NodeWelcome message sent in response to NodeHello.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeWelcome {
    /// Responder's NodeId.
    pub node_id: NodeId,
    /// Current epoch.
    pub epoch: Epoch,
    /// Current public key.
    pub public_key: PublicKey,
    /// Current timestamp.
    pub timestamp: Timestamp,
    /// Accepted features (intersection).
    pub features: u64,
}
