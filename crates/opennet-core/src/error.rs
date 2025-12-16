//! Core error types for OpenNet.

use thiserror::Error;

/// Result type alias for core operations.
pub type Result<T> = std::result::Result<T, CoreError>;

/// Core error types.
#[derive(Debug, Error)]
pub enum CoreError {
    /// Invalid NodeId format or derivation.
    #[error("invalid node id: {0}")]
    InvalidNodeId(String),

    /// Invalid ServiceId format.
    #[error("invalid service id: {0}")]
    InvalidServiceId(String),

    /// Invalid epoch number or state.
    #[error("invalid epoch: {0}")]
    InvalidEpoch(String),

    /// URI parsing error.
    #[error("invalid uri: {0}")]
    InvalidUri(String),

    /// Scope parsing error.
    #[error("invalid scope: {0}")]
    InvalidScope(String),

    /// Cryptographic operation failed.
    #[error("crypto error: {0}")]
    CryptoError(String),

    /// Serialization error.
    #[error("serialization error: {0}")]
    SerializationError(String),
}
