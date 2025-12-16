//! Time error types.

use thiserror::Error;

pub type Result<T> = std::result::Result<T, TimeError>;

#[derive(Debug, Error)]
pub enum TimeError {
    #[error("drift exceeded: {0}s")]
    DriftExceeded(u64),
    #[error("epoch expired")]
    EpochExpired,
    #[error("replay detected: nonce {0}")]
    ReplayDetected(u64),
    #[error("invalid timestamp")]
    InvalidTimestamp,
    #[error("insufficient samples")]
    InsufficientSamples,
}
