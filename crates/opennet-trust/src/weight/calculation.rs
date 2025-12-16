//! Trust weight calculation.

use super::TrustWeight;
use crate::decay::exponential_decay;

/// Calculate effective trust weight with decay.
pub fn calculate_weight(base_weight: TrustWeight, epoch_age: u64, lambda: TrustWeight) -> TrustWeight {
    let decay = exponential_decay(epoch_age, lambda);
    base_weight.mul(decay)
}
