//! Scope - Logical network partitions.
//!
//! RFC: OpenNet Core Protocol §7

use serde::{Deserialize, Serialize};
use crate::error::{CoreError, Result};

/// Logical network partition scope.
///
/// Scopes provide:
/// - Geographic partitioning (eu, us, asia)
/// - Privacy levels (anon, private)
/// - Testing environments (test, dev)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Scope {
    /// Global scope (default).
    Global,
    /// Geographic region.
    Region(String),
    /// Privacy-enhanced scope.
    Privacy(PrivacyLevel),
    /// Testing/development scope.
    Test(String),
    /// Custom scope.
    Custom(String),
}

/// Privacy levels for scoped resolution.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PrivacyLevel {
    /// Standard privacy.
    Standard,
    /// Enhanced anonymity.
    Anonymous,
    /// Private network segment.
    Private,
}

impl Scope {
    /// Parse scope from string.
    ///
    /// Format: `type.value` (e.g., "region.eu", "privacy.anon")
    pub fn parse(s: &str) -> Result<Self> {
        if s.is_empty() || s == "global" {
            return Ok(Scope::Global);
        }

        let parts: Vec<&str> = s.splitn(2, '.').collect();
        match parts.as_slice() {
            ["region", region] => Ok(Scope::Region(region.to_string())),
            ["privacy", "anon"] => Ok(Scope::Privacy(PrivacyLevel::Anonymous)),
            ["privacy", "private"] => Ok(Scope::Privacy(PrivacyLevel::Private)),
            ["privacy", _] => Ok(Scope::Privacy(PrivacyLevel::Standard)),
            ["test", name] => Ok(Scope::Test(name.to_string())),
            [single] => Ok(Scope::Custom(single.to_string())),
            _ => Err(CoreError::InvalidScope(s.to_string())),
        }
    }

    /// Convert scope to string representation.
    pub fn to_string_repr(&self) -> String {
        match self {
            Scope::Global => "global".to_string(),
            Scope::Region(r) => format!("region.{}", r),
            Scope::Privacy(PrivacyLevel::Anonymous) => "privacy.anon".to_string(),
            Scope::Privacy(PrivacyLevel::Private) => "privacy.private".to_string(),
            Scope::Privacy(PrivacyLevel::Standard) => "privacy.standard".to_string(),
            Scope::Test(t) => format!("test.{}", t),
            Scope::Custom(c) => c.clone(),
        }
    }

    /// Check if this scope matches another (for filtering).
    pub fn matches(&self, other: &Scope) -> bool {
        match (self, other) {
            (Scope::Global, _) => true,
            (_, Scope::Global) => true,
            (a, b) => a == b,
        }
    }
}

impl Default for Scope {
    fn default() -> Self {
        Scope::Global
    }
}

impl std::fmt::Display for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string_repr())
    }
}
