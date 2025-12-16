use thiserror::Error;
pub type Result<T> = std::result::Result<T, ResolverError>;

#[derive(Debug, Error)]
pub enum ResolverError {
    #[error("resolution failed: {0}")]
    ResolutionFailed(String),
    #[error("no candidates found")]
    NoCandidates,
    #[error("invalid uri: {0}")]
    InvalidUri(String),
    #[error("scope mismatch")]
    ScopeMismatch,
}
