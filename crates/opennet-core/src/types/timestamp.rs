//! Timestamp type (NOT system time).
//!
//! This is a protocol timestamp, not a direct system time access.

use serde::{Deserialize, Serialize};

/// Protocol timestamp (seconds since UNIX epoch).
///
/// This type represents timestamps exchanged in the protocol.
/// It does NOT directly access system time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Timestamp(pub u64);

impl Timestamp {
    /// Create a new timestamp.
    pub fn new(secs: u64) -> Self {
        Self(secs)
    }

    /// Get seconds since epoch.
    pub fn as_secs(&self) -> u64 {
        self.0
    }

    /// Zero timestamp.
    pub const ZERO: Self = Self(0);

    /// Check if timestamp is within range of another.
    pub fn within_range(&self, other: Timestamp, tolerance_secs: u64) -> bool {
        let diff = if self.0 > other.0 {
            self.0 - other.0
        } else {
            other.0 - self.0
        };
        diff <= tolerance_secs
    }

    /// Add seconds.
    pub fn add_secs(&self, secs: u64) -> Self {
        Self(self.0.saturating_add(secs))
    }

    /// Subtract seconds.
    pub fn sub_secs(&self, secs: u64) -> Self {
        Self(self.0.saturating_sub(secs))
    }
}

impl Default for Timestamp {
    fn default() -> Self {
        Self::ZERO
    }
}

impl std::fmt::Display for Timestamp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
