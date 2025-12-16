//! Fixed-point trust weight (NO FLOATING POINT).

use fixed::types::I32F32;
use serde::{Deserialize, Serialize};

/// Trust weight using fixed-point arithmetic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TrustWeight(i64); // Stored as fixed-point * 1_000_000

impl TrustWeight {
    /// Zero trust.
    pub const ZERO: Self = Self(0);
    /// Maximum trust (1.0).
    pub const MAX: Self = Self(1_000_000);
    /// Initial trust (0.01).
    pub const INITIAL: Self = Self(10_000);

    /// Create from fixed-point value.
    pub fn from_raw(raw: i64) -> Self {
        Self(raw.clamp(0, 1_000_000))
    }

    /// Create from decimal (e.g., 0.5).
    pub fn from_f64(value: f64) -> Self {
        Self((value * 1_000_000.0) as i64)
    }

    /// Convert to f64 (for display only).
    pub fn to_f64(self) -> f64 {
        self.0 as f64 / 1_000_000.0
    }

    /// Get raw value.
    pub fn raw(self) -> i64 {
        self.0
    }

    /// Multiply weights.
    pub fn mul(self, other: Self) -> Self {
        Self::from_raw((self.0 * other.0) / 1_000_000)
    }

    /// Add weights (capped at 1.0).
    pub fn add(self, other: Self) -> Self {
        Self::from_raw((self.0 + other.0).min(1_000_000))
    }

    /// Subtract weights (floored at 0).
    pub fn sub(self, other: Self) -> Self {
        Self::from_raw((self.0 - other.0).max(0))
    }
}

impl Default for TrustWeight {
    fn default() -> Self {
        Self::INITIAL
    }
}
