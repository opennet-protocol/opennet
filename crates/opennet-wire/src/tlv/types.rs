//! TLV type registry.

/// TLV type identifiers.
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TlvType {
    /// Frame header.
    FrameHeader = 0x0001,
    /// CBOR payload.
    CborPayload = 0x0002,
    /// Signature block.
    Signature = 0x0003,
    /// Node identifier.
    NodeId = 0x0004,
    /// Epoch info.
    EpochInfo = 0x0005,
    /// Extension slot.
    Extension = 0x00FF,
    /// Padding (ignored).
    Padding = 0x0000,
}

impl TlvType {
    /// Get the u16 value.
    pub fn value(self) -> u16 {
        self as u16
    }

    /// Try to parse from u16.
    pub fn from_u16(v: u16) -> Option<Self> {
        match v {
            0x0001 => Some(Self::FrameHeader),
            0x0002 => Some(Self::CborPayload),
            0x0003 => Some(Self::Signature),
            0x0004 => Some(Self::NodeId),
            0x0005 => Some(Self::EpochInfo),
            0x00FF => Some(Self::Extension),
            0x0000 => Some(Self::Padding),
            _ => None,
        }
    }

    /// Check if this type is known.
    pub fn is_known(v: u16) -> bool {
        Self::from_u16(v).is_some()
    }
}
