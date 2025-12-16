//! Trust graph node.

use opennet_core::NodeId;
use crate::weight::TrustWeight;

/// Trust graph node.
#[derive(Debug, Clone)]
pub struct TrustNode {
    pub node_id: NodeId,
    pub weight: TrustWeight,
    pub created_epoch: u64,
    pub last_updated: u64,
}
