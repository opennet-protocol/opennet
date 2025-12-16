use opennet_core::NodeId;
use std::collections::BTreeSet;

pub fn calculate_diversity(selected: &BTreeSet<NodeId>, _candidate: &NodeId) -> f64 {
    1.0 / (1.0 + selected.len() as f64 * 0.1)
}
