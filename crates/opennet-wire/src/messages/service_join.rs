//! ServiceJoin message - announce service participation.

use opennet_core::{NodeId, ServiceId};
use opennet_core::types::Timestamp;
use serde::{Deserialize, Serialize};

/// ServiceJoin message to announce service participation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceJoin {
    /// Node joining the service.
    pub node_id: NodeId,
    /// Service being joined.
    pub service_id: ServiceId,
    /// Join timestamp.
    pub timestamp: Timestamp,
    /// Service-specific metadata.
    pub metadata: Option<Vec<u8>>,
}
