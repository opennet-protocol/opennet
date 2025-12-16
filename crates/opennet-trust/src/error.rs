//! Trust error types.

use thiserror::Error;

pub type Result<T> = std::result::Result<T, TrustError>;

#[derive(Debug, Error)]
pub enum TrustError {
    #[error("trust below threshold: {0}")]
    BelowThreshold(String),
    #[error("node not found: {0}")]
    NodeNotFound(String),
    #[error("invalid trust weight: {0}")]
    InvalidWeight(String),
    #[error("graph error: {0}")]
    GraphError(String),
}
