//! Frame payload handling.

/// Frame payload wrapper.
#[derive(Debug, Clone)]
pub struct FramePayload {
    /// Raw CBOR bytes.
    pub cbor_data: Vec<u8>,
}

impl FramePayload {
    /// Create from CBOR bytes.
    pub fn from_cbor(data: Vec<u8>) -> Self {
        Self { cbor_data: data }
    }

    /// Get CBOR bytes.
    pub fn as_bytes(&self) -> &[u8] {
        &self.cbor_data
    }

    /// Into CBOR bytes.
    pub fn into_bytes(self) -> Vec<u8> {
        self.cbor_data
    }
}
