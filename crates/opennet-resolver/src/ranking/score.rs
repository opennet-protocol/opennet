use crate::candidates::NodeDescriptor;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct RankScore(pub i64);

impl RankScore {
    /// RankScore = TW × ScopeAffinity × DiversityFactor
    pub fn calculate(trust_weight: f64, scope_affinity: f64, diversity: f64) -> Self {
        Self(((trust_weight * scope_affinity * diversity) * 1_000_000.0) as i64)
    }
}

pub fn rank_candidates(mut candidates: Vec<NodeDescriptor>) -> Vec<NodeDescriptor> {
    candidates.sort_by(|a, b| b.rank_score.0.cmp(&a.rank_score.0));
    candidates
}
