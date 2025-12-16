//! Signature wrapper type.

use serde::{Deserialize, Serialize};

/// Ed25519 signature (64 bytes).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Signature(pub [u8; 64]);

impl Signature {
    /// Create from bytes.
    pub fn from_bytes(bytes: [u8; 64]) -> Self {
        Self(bytes)
    }

    /// Get as bytes.
    pub fn as_bytes(&self) -> &[u8; 64] {
        &self.0
    }

    /// Convert to hex string.
    pub fn to_hex(&self) -> String {
        hex::encode(self.0)
    }
}

impl AsRef<[u8]> for Signature {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}
