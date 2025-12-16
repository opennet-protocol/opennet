//! ACTIVE state behavior.

use crate::fsm::StateEvent;

/// Active state handler.
pub struct ActiveState {
    current_trust: f64,
    warn_threshold: f64,
    critical_threshold: f64,
}

impl ActiveState {
    pub fn new(warn_threshold: f64, critical_threshold: f64) -> Self {
        Self {
            current_trust: 1.0,
            warn_threshold,
            critical_threshold,
        }
    }

    /// Update trust and check thresholds.
    pub fn update_trust(&mut self, trust: f64) -> Option<StateEvent> {
        self.current_trust = trust;
        
        if trust < self.critical_threshold {
            Some(StateEvent::TrustCritical)
        } else if trust < self.warn_threshold {
            Some(StateEvent::TrustBelowWarn)
        } else {
            None
        }
    }

    /// Check if can relay traffic.
    pub fn can_relay(&self) -> bool {
        self.current_trust >= 0.4 // RELAY_THRESHOLD
    }
}

impl Default for ActiveState {
    fn default() -> Self {
        Self::new(0.15, 0.05)
    }
}
