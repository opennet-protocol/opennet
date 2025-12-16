//! ServiceId - Cryptographic service identifier.
//!
//! RFC: OpenNet Core Protocol §5

use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use crate::constants::SERVICE_ID_LEN;
use crate::error::{CoreError, Result};

/// Cryptographic service identifier.
///
/// ServiceId binds a human-readable domain to a cryptographic identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ServiceId([u8; SERVICE_ID_LEN]);

impl ServiceId {
    /// Create a ServiceId from raw bytes.
    pub fn from_bytes(bytes: [u8; SERVICE_ID_LEN]) -> Self {
        Self(bytes)
    }

    /// Derive ServiceId from domain name.
    ///
    /// ```text
    /// ServiceId = SHA256(normalized_domain)
    /// ```
    pub fn from_domain(domain: &str) -> Self {
        let normalized = domain.to_lowercase();
        let mut hasher = Sha256::new();
        hasher.update(normalized.as_bytes());
        let result = hasher.finalize();
        let mut bytes = [0u8; SERVICE_ID_LEN];
        bytes.copy_from_slice(&result);
        Self(bytes)
    }

    /// Parse ServiceId from hex string.
    pub fn from_hex(hex: &str) -> Result<Self> {
        let bytes = hex::decode(hex)
            .map_err(|e| CoreError::InvalidServiceId(e.to_string()))?;
        if bytes.len() != SERVICE_ID_LEN {
            return Err(CoreError::InvalidServiceId("invalid length".into()));
        }
        let mut arr = [0u8; SERVICE_ID_LEN];
        arr.copy_from_slice(&bytes);
        Ok(Self(arr))
    }

    /// Get raw bytes.
    pub fn as_bytes(&self) -> &[u8; SERVICE_ID_LEN] {
        &self.0
    }

    /// Convert to hex string.
    pub fn to_hex(&self) -> String {
        hex::encode(self.0)
    }
}

impl std::fmt::Display for ServiceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.to_hex()[..16])
    }
}

impl Ord for ServiceId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for ServiceId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
