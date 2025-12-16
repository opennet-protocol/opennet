//! Outlier detection for time samples.

use opennet_core::types::Timestamp;

/// Detect and filter outlier time samples.
pub struct OutlierDetector {
    tolerance_secs: u64,
}

impl OutlierDetector {
    pub fn new(tolerance_secs: u64) -> Self {
        Self { tolerance_secs }
    }

    /// Filter outliers from samples.
    pub fn filter(&self, samples: &[Timestamp], reference: Timestamp) -> Vec<Timestamp> {
        samples.iter()
            .filter(|s| s.within_range(reference, self.tolerance_secs))
            .copied()
            .collect()
    }
}

impl Default for OutlierDetector {
    fn default() -> Self {
        Self::new(300) // 5 minutes
    }
}
