//! Decay parameters.

use crate::weight::TrustWeight;

/// Decay parameters.
pub struct DecayParameters {
    /// Lambda (decay constant).
    pub lambda: TrustWeight,
}

impl Default for DecayParameters {
    fn default() -> Self {
        Self { lambda: TrustWeight::from_f64(0.05) }
    }
}
