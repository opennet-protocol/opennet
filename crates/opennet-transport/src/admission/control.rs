use opennet_core::NodeId;
use crate::error::{TransportError, Result};

pub struct AdmissionControl { threshold: f64 }

impl AdmissionControl {
    pub fn new(threshold: f64) -> Self { Self { threshold } }
    pub fn check(&self, _node_id: &NodeId, trust_weight: f64) -> Result<()> {
        if trust_weight < self.threshold {
            return Err(TransportError::TrustTooLow);
        }
        Ok(())
    }
}

impl Default for AdmissionControl {
    fn default() -> Self { Self::new(0.3) }
}
