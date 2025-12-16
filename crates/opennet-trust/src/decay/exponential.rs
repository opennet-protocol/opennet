//! Exponential decay function.
//!
//! decay(t) = e^(-λt)
//!
//! Using integer approximation to avoid floating point.

use crate::weight::TrustWeight;

/// Calculate exponential decay using integer math.
///
/// Approximation: e^(-λt) ≈ 1 / (1 + λt + (λt)²/2)
pub fn exponential_decay(epoch_age: u64, lambda: TrustWeight) -> TrustWeight {
    if epoch_age == 0 {
        return TrustWeight::MAX;
    }

    // Simple approximation: decay by lambda per epoch
    let lambda_raw = lambda.raw();
    let decay_factor = 1_000_000i64.saturating_sub(lambda_raw * epoch_age as i64);
    
    TrustWeight::from_raw(decay_factor.max(0))
}
