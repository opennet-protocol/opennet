use thiserror::Error;
pub type Result<T> = std::result::Result<T, NodeError>;

#[derive(Debug, Error)]
pub enum NodeError {
    #[error("invalid state transition: {from:?} -> {to:?}")]
    InvalidTransition { from: String, to: String },
    #[error("fsm error: {0}")]
    FsmError(String),
    #[error("sync failed: {0}")]
    SyncFailed(String),
    #[error("peer error: {0}")]
    PeerError(String),
}
