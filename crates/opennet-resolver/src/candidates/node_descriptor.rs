use opennet_core::{NodeId, Scope};
use crate::ranking::RankScore;

#[derive(Debug, Clone)]
pub struct NodeDescriptor {
    pub node_id: NodeId,
    pub scope: Scope,
    pub trust_weight: f64,
    pub rank_score: RankScore,
    pub endpoint: String,
}
