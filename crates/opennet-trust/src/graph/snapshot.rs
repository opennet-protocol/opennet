//! Graph snapshot for deterministic state.

use super::TrustGraph;

/// Immutable snapshot of trust graph.
#[derive(Debug, Clone)]
pub struct GraphSnapshot {
    pub epoch: u64,
    pub node_count: usize,
    pub edge_count: usize,
}

impl GraphSnapshot {
    pub fn from_graph(graph: &TrustGraph, epoch: u64) -> Self {
        Self {
            epoch,
            node_count: graph.node_count(),
            edge_count: graph.edge_count(),
        }
    }
}
