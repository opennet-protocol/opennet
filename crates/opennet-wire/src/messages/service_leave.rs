//! ServiceLeave message - announce service departure.

use opennet_core::{NodeId, ServiceId};
use opennet_core::types::Timestamp;
use serde::{Deserialize, Serialize};

/// ServiceLeave message to announce service departure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceLeave {
    /// Node leaving the service.
    pub node_id: NodeId,
    /// Service being left.
    pub service_id: ServiceId,
    /// Leave timestamp.
    pub timestamp: Timestamp,
    /// Reason code.
    pub reason: u8,
}
