//! Identity error types.

use thiserror::Error;

/// Result type for identity operations.
pub type Result<T> = std::result::Result<T, IdentityError>;

/// Identity errors.
#[derive(Debug, Error)]
pub enum IdentityError {
    /// Invalid key format.
    #[error("invalid key: {0}")]
    InvalidKey(String),

    /// Key rotation failed.
    #[error("rotation failed: {0}")]
    RotationFailed(String),

    /// Epoch chain broken.
    #[error("epoch chain broken: {0}")]
    EpochChainBroken(String),

    /// Signature verification failed.
    #[error("signature verification failed")]
    SignatureVerificationFailed,

    /// Key compromise detected.
    #[error("key compromise detected: {0}")]
    CompromiseDetected(String),

    /// Storage error.
    #[error("storage error: {0}")]
    StorageError(String),

    /// Cryptographic error.
    #[error("crypto error: {0}")]
    CryptoError(String),
}
