//! DEGRADED state behavior.

use crate::fsm::StateEvent;

/// Degraded state handler.
pub struct DegradedState {
    current_trust: f64,
    ok_threshold: f64,
    critical_threshold: f64,
}

impl DegradedState {
    pub fn new(ok_threshold: f64, critical_threshold: f64) -> Self {
        Self {
            current_trust: 0.1,
            ok_threshold,
            critical_threshold,
        }
    }

    /// Update trust and check for recovery or further degradation.
    pub fn update_trust(&mut self, trust: f64) -> Option<StateEvent> {
        self.current_trust = trust;
        
        if trust < self.critical_threshold {
            Some(StateEvent::TrustCritical)
        } else if trust >= self.ok_threshold {
            Some(StateEvent::TrustRecovered)
        } else {
            None
        }
    }
}

impl Default for DegradedState {
    fn default() -> Self {
        Self::new(0.2, 0.05)
    }
}
