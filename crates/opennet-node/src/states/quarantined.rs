//! QUARANTINED state behavior (terminal).

/// Quarantined state handler.
pub struct QuarantinedState {
    reason: String,
}

impl QuarantinedState {
    pub fn new(reason: String) -> Self {
        Self { reason }
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    /// Quarantined is terminal - no recovery possible.
    pub fn can_recover(&self) -> bool {
        false
    }
}

impl Default for QuarantinedState {
    fn default() -> Self {
        Self::new("unknown".to_string())
    }
}
