//! TrustEvaluator - FSM event producer.

use opennet_core::NodeId;
use crate::graph::TrustGraph;
use crate::thresholds::Thresholds;

/// Events produced by TrustEvaluator.
#[derive(Debug, Clone)]
pub enum TrustEvent {
    TrustBelowWarn { node_id: NodeId },
    TrustBelowCritical { node_id: NodeId },
    TrustRecovered { node_id: NodeId },
}

/// Evaluates trust and produces FSM events.
pub struct TrustEvaluator {
    thresholds: Thresholds,
    pending_events: Vec<TrustEvent>,
}

impl TrustEvaluator {
    pub fn new(thresholds: Thresholds) -> Self {
        Self { thresholds, pending_events: Vec::new() }
    }

    /// Evaluate a node's trust.
    pub fn evaluate(&mut self, graph: &TrustGraph, node_id: &NodeId) {
        if let Some(weight) = graph.get_weight(node_id) {
            let weight_f64 = weight.to_f64();
            
            if weight_f64 < self.thresholds.critical {
                self.pending_events.push(TrustEvent::TrustBelowCritical { node_id: *node_id });
            } else if weight_f64 < self.thresholds.warn {
                self.pending_events.push(TrustEvent::TrustBelowWarn { node_id: *node_id });
            }
        }
    }

    pub fn drain_events(&mut self) -> Vec<TrustEvent> {
        std::mem::take(&mut self.pending_events)
    }
}
