//! TLV writer.

use super::frame::TlvFrame;
use super::types::TlvType;
use crate::error::Result;

/// TLV frame writer.
pub struct TlvWriter {
    buffer: Vec<u8>,
}

impl TlvWriter {
    /// Create new writer.
    pub fn new() -> Self {
        Self { buffer: Vec::new() }
    }

    /// Write a frame.
    pub fn write_frame(&mut self, frame: &TlvFrame) -> &mut Self {
        self.buffer.extend_from_slice(&frame.to_bytes());
        self
    }

    /// Write a typed frame.
    pub fn write(&mut self, frame_type: TlvType, value: &[u8]) -> Result<&mut Self> {
        let frame = TlvFrame::new(frame_type, value.to_vec())?;
        self.buffer.extend_from_slice(&frame.to_bytes());
        Ok(self)
    }

    /// Get written bytes.
    pub fn into_bytes(self) -> Vec<u8> {
        self.buffer
    }

    /// Get current length.
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    /// Check if empty.
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
}

impl Default for TlvWriter {
    fn default() -> Self {
        Self::new()
    }
}
