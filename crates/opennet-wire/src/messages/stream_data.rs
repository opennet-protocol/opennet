//! StreamData message - stream payload.

use serde::{Deserialize, Serialize};

/// StreamData message carrying stream payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamData {
    /// Stream identifier.
    pub stream_id: u64,
    /// Sequence number.
    pub sequence: u64,
    /// Payload data.
    pub payload: Vec<u8>,
    /// End-of-stream flag.
    pub fin: bool,
}
