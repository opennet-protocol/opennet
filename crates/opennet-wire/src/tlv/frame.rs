//! TLV frame structure.

use super::types::TlvType;
use crate::error::{WireError, Result};

/// Maximum frame size (64 KB).
pub const MAX_FRAME_SIZE: usize = 65536;

/// TLV frame structure.
///
/// ```text
/// [ Type (u16) | Length (u32) | Value (bytes) ]
/// ```
#[derive(Debug, Clone)]
pub struct TlvFrame {
    /// Frame type.
    pub frame_type: u16,
    /// Frame value.
    pub value: Vec<u8>,
}

impl TlvFrame {
    /// Create a new frame.
    pub fn new(frame_type: TlvType, value: Vec<u8>) -> Result<Self> {
        if value.len() > MAX_FRAME_SIZE {
            return Err(WireError::FrameTooLarge(value.len()));
        }
        Ok(Self {
            frame_type: frame_type.value(),
            value,
        })
    }

    /// Create from raw type and value.
    pub fn from_raw(frame_type: u16, value: Vec<u8>) -> Result<Self> {
        if value.len() > MAX_FRAME_SIZE {
            return Err(WireError::FrameTooLarge(value.len()));
        }
        Ok(Self { frame_type, value })
    }

    /// Get frame type as enum (if known).
    pub fn get_type(&self) -> Option<TlvType> {
        TlvType::from_u16(self.frame_type)
    }

    /// Serialize to bytes.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(6 + self.value.len());
        buf.extend_from_slice(&self.frame_type.to_be_bytes());
        buf.extend_from_slice(&(self.value.len() as u32).to_be_bytes());
        buf.extend_from_slice(&self.value);
        buf
    }

    /// Parse from bytes.
    pub fn from_bytes(data: &[u8]) -> Result<(Self, usize)> {
        if data.len() < 6 {
            return Err(WireError::InvalidTlv("too short".into()));
        }

        let frame_type = u16::from_be_bytes([data[0], data[1]]);
        let length = u32::from_be_bytes([data[2], data[3], data[4], data[5]]) as usize;

        if data.len() < 6 + length {
            return Err(WireError::InvalidTlv("incomplete frame".into()));
        }

        let value = data[6..6 + length].to_vec();
        let frame = Self::from_raw(frame_type, value)?;

        Ok((frame, 6 + length))
    }
}
