//! Trust propagation model.

use super::TrustWeight;
use crate::graph::TrustEdge;

/// Propagate trust through an edge.
pub fn propagate_trust(source_weight: TrustWeight, edge: &TrustEdge) -> TrustWeight {
    source_weight.mul(edge.weight)
}
