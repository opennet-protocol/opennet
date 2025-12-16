//! Trust graph storage.

use opennet_core::NodeId;
use super::edge::TrustEdge;
use crate::weight::TrustWeight;
use indexmap::IndexMap;
use std::collections::BTreeMap;

/// Trust graph G = (V, E).
pub struct TrustGraph {
    /// Node weights.
    weights: BTreeMap<NodeId, TrustWeight>,
    /// Edges: source -> (target -> edge).
    edges: BTreeMap<NodeId, IndexMap<NodeId, TrustEdge>>,
}

impl TrustGraph {
    pub fn new() -> Self {
        Self { weights: BTreeMap::new(), edges: BTreeMap::new() }
    }

    /// Add or update a node.
    pub fn upsert_node(&mut self, node_id: NodeId, weight: TrustWeight) {
        self.weights.insert(node_id, weight);
    }

    /// Add or update an edge.
    pub fn upsert_edge(&mut self, edge: TrustEdge) {
        self.edges
            .entry(edge.source)
            .or_default()
            .insert(edge.target, edge);
    }

    /// Get node weight.
    pub fn get_weight(&self, node_id: &NodeId) -> Option<TrustWeight> {
        self.weights.get(node_id).copied()
    }

    /// Get edges from a node.
    pub fn get_edges(&self, source: &NodeId) -> Option<&IndexMap<NodeId, TrustEdge>> {
        self.edges.get(source)
    }

    /// Get all nodes.
    pub fn nodes(&self) -> impl Iterator<Item = &NodeId> {
        self.weights.keys()
    }

    /// Node count.
    pub fn node_count(&self) -> usize {
        self.weights.len()
    }

    /// Edge count.
    pub fn edge_count(&self) -> usize {
        self.edges.values().map(|e| e.len()).sum()
    }
}

impl Default for TrustGraph {
    fn default() -> Self {
        Self::new()
    }
}
