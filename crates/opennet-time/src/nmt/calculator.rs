//! NMT calculation.

use opennet_core::types::Timestamp;
use crate::error::{TimeError, Result};

/// Calculates Network Median Time from peer samples.
pub struct NmtCalculator {
    min_samples: usize,
}

impl NmtCalculator {
    pub fn new(min_samples: usize) -> Self {
        Self { min_samples }
    }

    /// Calculate NMT from samples.
    pub fn calculate(&self, samples: &[Timestamp]) -> Result<Timestamp> {
        if samples.len() < self.min_samples {
            return Err(TimeError::InsufficientSamples);
        }

        let mut values: Vec<u64> = samples.iter().map(|t| t.as_secs()).collect();
        values.sort_unstable();

        let mid = values.len() / 2;
        let median = if values.len() % 2 == 0 {
            (values[mid - 1] + values[mid]) / 2
        } else {
            values[mid]
        };

        Ok(Timestamp::new(median))
    }
}

impl Default for NmtCalculator {
    fn default() -> Self {
        Self::new(3)
    }
}
