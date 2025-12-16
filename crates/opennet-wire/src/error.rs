//! Wire format error types.

use thiserror::Error;

/// Result type for wire operations.
pub type Result<T> = std::result::Result<T, WireError>;

/// Wire format errors.
#[derive(Debug, Error)]
pub enum WireError {
    /// Non-canonical CBOR encoding.
    #[error("non-canonical encoding: {0}")]
    NonCanonical(String),

    /// Invalid CBOR structure.
    #[error("invalid cbor: {0}")]
    InvalidCbor(String),

    /// Invalid TLV frame.
    #[error("invalid tlv: {0}")]
    InvalidTlv(String),

    /// Unknown message type.
    #[error("unknown message type: {0}")]
    UnknownMessageType(u16),

    /// Frame too large.
    #[error("frame too large: {0} bytes")]
    FrameTooLarge(usize),

    /// Duplicate map key.
    #[error("duplicate map key: {0}")]
    DuplicateKey(i64),

    /// Invalid key order.
    #[error("keys not in ascending order")]
    KeyOrderViolation,

    /// Floating point value detected.
    #[error("floating point values not allowed")]
    FloatingPointDetected,

    /// IO error.
    #[error("io error: {0}")]
    IoError(#[from] std::io::Error),
}
