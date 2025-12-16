//! NodeId - Unique, immutable node identifier.
//!
//! RFC: OpenNet Core Protocol §4

use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use crate::constants::NODE_ID_LEN;
use crate::error::{CoreError, Result};

/// Unique, immutable node identifier.
///
/// Derived from SHA-256 hash of the node's canonical public key.
/// This value NEVER changes throughout the node's lifetime.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId([u8; NODE_ID_LEN]);

impl NodeId {
    /// Create a NodeId from raw bytes.
    pub fn from_bytes(bytes: [u8; NODE_ID_LEN]) -> Self {
        Self(bytes)
    }

    /// Derive NodeId from a public key.
    ///
    /// ```text
    /// NodeId = SHA256(canonical_public_key)
    /// ```
    pub fn from_public_key(public_key: &[u8]) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(public_key);
        let result = hasher.finalize();
        let mut bytes = [0u8; NODE_ID_LEN];
        bytes.copy_from_slice(&result);
        Self(bytes)
    }

    /// Parse NodeId from hex string.
    pub fn from_hex(hex: &str) -> Result<Self> {
        let bytes = hex::decode(hex)
            .map_err(|e| CoreError::InvalidNodeId(e.to_string()))?;
        if bytes.len() != NODE_ID_LEN {
            return Err(CoreError::InvalidNodeId("invalid length".into()));
        }
        let mut arr = [0u8; NODE_ID_LEN];
        arr.copy_from_slice(&bytes);
        Ok(Self(arr))
    }

    /// Get raw bytes.
    pub fn as_bytes(&self) -> &[u8; NODE_ID_LEN] {
        &self.0
    }

    /// Convert to hex string.
    pub fn to_hex(&self) -> String {
        hex::encode(self.0)
    }
}

impl std::fmt::Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.to_hex()[..16]) // Short form
    }
}

impl AsRef<[u8]> for NodeId {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Ord for NodeId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for NodeId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
