use thiserror::Error;
pub type Result<T> = std::result::Result<T, RevocationError>;

#[derive(Debug, Error)]
pub enum RevocationError {
    #[error("invalid revocation: {0}")]
    Invalid(String),
    #[error("quorum not met")]
    QuorumNotMet,
    #[error("recovery failed: {0}")]
    RecoveryFailed(String),
}
