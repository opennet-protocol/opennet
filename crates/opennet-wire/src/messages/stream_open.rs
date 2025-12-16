//! StreamOpen message - open data stream.

use opennet_core::ServiceId;
use serde::{Deserialize, Serialize};

/// StreamOpen message to initiate a data stream.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamOpen {
    /// Stream identifier.
    pub stream_id: u64,
    /// Target service.
    pub service_id: ServiceId,
    /// Requested path.
    pub path: String,
    /// Initial window size.
    pub window_size: u32,
}
