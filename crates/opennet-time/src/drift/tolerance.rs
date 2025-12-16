//! Drift tolerance configuration.

/// Drift tolerance thresholds.
pub struct DriftTolerance {
    /// Warning threshold in seconds.
    pub warn_secs: u64,
    /// Critical threshold in seconds.
    pub critical_secs: u64,
}

impl DriftTolerance {
    pub fn new(warn_secs: u64, critical_secs: u64) -> Self {
        Self { warn_secs, critical_secs }
    }

    /// Check if drift is acceptable.
    pub fn is_acceptable(&self, drift_secs: u64) -> bool {
        drift_secs <= self.critical_secs
    }

    /// Check if drift is in warning range.
    pub fn is_warning(&self, drift_secs: u64) -> bool {
        drift_secs > self.warn_secs && drift_secs <= self.critical_secs
    }
}

impl Default for DriftTolerance {
    fn default() -> Self {
        Self::new(180, 300) // 3 min warn, 5 min critical
    }
}
