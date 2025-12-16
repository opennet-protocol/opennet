//! EpochMonitor - FSM event producer for time events.

use opennet_core::types::Timestamp;
use crate::clock::MonotonicClock;
use crate::drift::DriftDetector;
use crate::error::Result;

/// Events produced by EpochMonitor.
#[derive(Debug, Clone)]
pub enum TimeEvent {
    /// Drift warning threshold exceeded.
    DriftWarning { drift_secs: u64 },
    /// Drift critical threshold exceeded.
    DriftCritical { drift_secs: u64 },
    /// Replay attack detected.
    ReplayDetected { nonce: u64 },
    /// Epoch about to expire.
    EpochExpiringSoon { remaining_secs: u64 },
}

/// Monitors time and epoch state.
pub struct EpochMonitor<C: MonotonicClock> {
    clock: C,
    drift_detector: DriftDetector,
    pending_events: Vec<TimeEvent>,
    warn_threshold: u64,
    critical_threshold: u64,
}

impl<C: MonotonicClock> EpochMonitor<C> {
    pub fn new(clock: C) -> Self {
        Self {
            clock,
            drift_detector: DriftDetector::new(),
            pending_events: Vec::new(),
            warn_threshold: 180,  // 3 minutes
            critical_threshold: 300,  // 5 minutes
        }
    }

    /// Check drift against peer timestamp.
    pub fn check_drift(&mut self, peer_timestamp: Timestamp) -> Result<()> {
        let local = self.clock.now();
        let drift = self.drift_detector.calculate_drift(local, peer_timestamp);
        
        if drift >= self.critical_threshold {
            self.pending_events.push(TimeEvent::DriftCritical { drift_secs: drift });
        } else if drift >= self.warn_threshold {
            self.pending_events.push(TimeEvent::DriftWarning { drift_secs: drift });
        }
        
        Ok(())
    }

    /// Drain pending events.
    pub fn drain_events(&mut self) -> Vec<TimeEvent> {
        std::mem::take(&mut self.pending_events)
    }
}
