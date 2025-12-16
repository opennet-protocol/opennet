//! Revocation message.

use opennet_core::NodeId;
use opennet_core::types::{Signature, Timestamp};
use serde::{Deserialize, Serialize};

/// Revocation message announcing key compromise.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevocationMessage {
    /// Node being revoked.
    pub node_id: NodeId,
    /// Revoked epoch ID.
    pub revoked_epoch: u64,
    /// Reason code.
    pub reason: RevocationReason,
    /// Revocation timestamp.
    pub timestamp: Timestamp,
    /// Self-signature or quorum signatures.
    pub signatures: Vec<RevocationSignature>,
}

/// Revocation reason codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RevocationReason {
    /// Key was compromised.
    KeyCompromise = 0,
    /// Key was forcibly disclosed.
    ForcedDisclosure = 1,
    /// Detected unauthorized usage.
    UnauthorizedUsage = 2,
    /// Voluntary revocation.
    Voluntary = 3,
}

/// Signature on a revocation message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevocationSignature {
    /// Signer NodeId.
    pub signer: NodeId,
    /// Signer's epoch.
    pub epoch_id: u64,
    /// Signature.
    pub signature: Signature,
}
