//! Drift detection.

use opennet_core::types::Timestamp;

/// Detects clock drift between nodes.
pub struct DriftDetector {
    samples: Vec<i64>,
    max_samples: usize,
}

impl DriftDetector {
    pub fn new() -> Self {
        Self { samples: Vec::new(), max_samples: 20 }
    }

    /// Calculate drift between local and peer timestamp.
    pub fn calculate_drift(&mut self, local: Timestamp, peer: Timestamp) -> u64 {
        let drift = (local.as_secs() as i64) - (peer.as_secs() as i64);
        self.samples.push(drift);
        
        if self.samples.len() > self.max_samples {
            self.samples.remove(0);
        }
        
        drift.unsigned_abs()
    }

    /// Get average drift.
    pub fn average_drift(&self) -> i64 {
        if self.samples.is_empty() {
            return 0;
        }
        self.samples.iter().sum::<i64>() / self.samples.len() as i64
    }
}

impl Default for DriftDetector {
    fn default() -> Self {
        Self::new()
    }
}
