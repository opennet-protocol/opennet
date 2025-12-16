//! Wire format validation.

use crate::cbor::CanonicalCbor;
use crate::tlv::TlvReader;
use crate::error::{WireError, Result};

/// Validate a complete wire frame.
pub fn validate_frame(data: &[u8]) -> Result<()> {
    // Parse TLV frames
    let mut reader = TlvReader::new(data);
    let frames = reader.read_all()?;

    if frames.is_empty() {
        return Err(WireError::InvalidTlv("empty frame".into()));
    }

    // Validate each CBOR payload
    for frame in &frames {
        if frame.frame_type == 0x0002 {
            // CBOR payload
            CanonicalCbor::validate(&frame.value)?;
        }
    }

    Ok(())
}

/// Validate message type is known.
pub fn validate_message_type(msg_type: u16) -> Result<()> {
    match msg_type {
        0x0001..=0x0002 | 0x0010..=0x0011 | 0x0020..=0x0022 | 0x0030 => Ok(()),
        _ => Err(WireError::UnknownMessageType(msg_type)),
    }
}
