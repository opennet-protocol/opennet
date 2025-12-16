//! Frame header structure.

use opennet_core::types::Timestamp;
use serde::{Deserialize, Serialize};

/// Frame header for all wire messages.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameHeader {
    /// Protocol version.
    pub version: u16,
    /// Message type.
    pub message_type: u16,
    /// Timestamp.
    pub timestamp: Timestamp,
    /// Sequence number.
    pub sequence: u64,
}

impl FrameHeader {
    /// Create new header.
    pub fn new(message_type: u16, timestamp: Timestamp, sequence: u64) -> Self {
        Self {
            version: opennet_core::WIRE_VERSION,
            message_type,
            timestamp,
            sequence,
        }
    }
}
