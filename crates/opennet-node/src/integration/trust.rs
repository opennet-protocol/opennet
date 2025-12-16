//! Trust integration.

use opennet_core::NodeId;
use opennet_trust::graph::TrustGraph;

pub struct TrustIntegration {
    graph: TrustGraph,
}

impl TrustIntegration {
    pub fn new() -> Self {
        Self { graph: TrustGraph::new() }
    }

    pub fn get_trust(&self, node_id: &NodeId) -> f64 {
        self.graph.get_weight(node_id)
            .map(|w| w.to_f64())
            .unwrap_or(0.0)
    }

    pub fn graph(&self) -> &TrustGraph {
        &self.graph
    }
}

impl Default for TrustIntegration {
    fn default() -> Self { Self::new() }
}
