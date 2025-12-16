//! StreamClose message - close data stream.

use serde::{Deserialize, Serialize};

/// StreamClose message to terminate a stream.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamClose {
    /// Stream identifier.
    pub stream_id: u64,
    /// Close reason code.
    pub reason: u32,
    /// Error message (optional).
    pub message: Option<String>,
}
