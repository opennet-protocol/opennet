//! Time sample collection.

use opennet_core::NodeId;
use opennet_core::types::Timestamp;
use std::collections::BTreeMap;

/// Collects time samples from peers.
pub struct TimeSampler {
    samples: BTreeMap<NodeId, Timestamp>,
    max_samples: usize,
}

impl TimeSampler {
    pub fn new(max_samples: usize) -> Self {
        Self { samples: BTreeMap::new(), max_samples }
    }

    /// Add a sample from a peer.
    pub fn add_sample(&mut self, node_id: NodeId, timestamp: Timestamp) {
        self.samples.insert(node_id, timestamp);
        
        // Evict oldest if over limit
        while self.samples.len() > self.max_samples {
            if let Some((&first_key, _)) = self.samples.iter().next() {
                self.samples.remove(&first_key);
            }
        }
    }

    /// Get all samples.
    pub fn samples(&self) -> Vec<Timestamp> {
        self.samples.values().copied().collect()
    }

    /// Clear all samples.
    pub fn clear(&mut self) {
        self.samples.clear();
    }
}

impl Default for TimeSampler {
    fn default() -> Self {
        Self::new(10)
    }
}
