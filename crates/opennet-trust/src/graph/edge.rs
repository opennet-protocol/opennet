//! Trust edge (weighted, directed).

use opennet_core::NodeId;
use crate::weight::TrustWeight;

/// Directed trust edge.
#[derive(Debug, Clone)]
pub struct TrustEdge {
    pub source: NodeId,
    pub target: NodeId,
    pub weight: TrustWeight,
    pub created_epoch: u64,
    pub last_updated: u64,
}

impl TrustEdge {
    pub fn new(source: NodeId, target: NodeId, weight: TrustWeight, epoch: u64) -> Self {
        Self { source, target, weight, created_epoch: epoch, last_updated: epoch }
    }
}
